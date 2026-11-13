//! Два паттерна unsafe, которые встречаются в actix-http, и их проверка.
use actix_web::http::header::HeaderValue;
use bytes::{BufMut, BytesMut};

// ---------- 1. Маскирование WebSocket-фрейма (как actix-http/src/ws/mask.rs) ----------

/// Эталон: побайтно, полностью безопасно.
pub fn mask_naive(buf: &mut [u8], mask: [u8; 4]) {
    for (i, b) in buf.iter_mut().enumerate() {
        *b ^= mask[i & 3];
    }
}

/// Быстрый путь: средняя выровненная часть обрабатывается словами u32.
pub fn mask_align_to(buf: &mut [u8], mask: [u8; 4]) {
    // SAFETY: любая последовательность из 4 байт — валидный u32 (нет «запрещённых»
    // битовых паттернов), а align_to_mut сам гарантирует выравнивание `words`
    // и то, что prefix/words/suffix не пересекаются и покрывают весь buf.
    let (prefix, words, suffix) = unsafe { buf.align_to_mut::<u32>() };
    mask_naive(prefix, mask);
    let head = prefix.len() & 3;
    let m = u32::from_ne_bytes(mask);
    let m = if head == 0 {
        m
    } else if cfg!(target_endian = "big") {
        m.rotate_left(8 * head as u32)
    } else {
        m.rotate_right(8 * head as u32)
    };
    for w in words.iter_mut() {
        *w ^= m;
    }
    mask_naive(suffix, m.to_ne_bytes());
}

/// Безопасная альтернатива: chunks_exact_mut + from_ne_bytes. Компилятор
/// часто генерирует для неё такой же векторизованный код.
pub fn mask_chunks(buf: &mut [u8], mask: [u8; 4]) {
    let m = u32::from_ne_bytes(mask);
    let mut chunks = buf.chunks_exact_mut(4);
    for c in &mut chunks {
        let w = u32::from_ne_bytes([c[0], c[1], c[2], c[3]]) ^ m;
        c.copy_from_slice(&w.to_ne_bytes());
    }
    mask_naive(chunks.into_remainder(), mask);
}

// ---------- 2. Заголовок без повторной валидации (как h1/encoder, h2/dispatcher) ----------

/// Число → HeaderValue. Проверку «только видимые ASCII» пропускаем:
/// цифры гарантированно ASCII, это инвариант нашей функции.
pub fn header_from_u64(n: u64) -> HeaderValue {
    let mut buf = BytesMut::with_capacity(20);
    buf.put_slice(&itoa_buf(n));
    debug_assert!(buf.iter().all(u8::is_ascii_digit));
    // SAFETY: buf содержит только b'0'..=b'9' (см. itoa_buf и debug_assert выше),
    // а это допустимые байты значения HTTP-заголовка.
    unsafe { HeaderValue::from_maybe_shared_unchecked(buf.freeze()) }
}

/// Безопасная версия для сравнения: валидирует каждый байт.
pub fn header_from_u64_checked(n: u64) -> HeaderValue {
    HeaderValue::from_str(&n.to_string()).expect("digits are valid")
}

fn itoa_buf(mut n: u64) -> Vec<u8> {
    if n == 0 {
        return vec![b'0'];
    }
    let mut out = Vec::with_capacity(20);
    while n > 0 {
        out.push(b'0' + (n % 10) as u8);
        n /= 10;
    }
    out.reverse();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Сверяем быстрые версии с эталоном на всех длинах и смещениях —
    /// именно смещение (невыровненное начало) ломает наивные «оптимизации».
    #[test]
    fn mask_versions_agree() {
        let data: Vec<u8> = (0..=255u8).cycle().take(1000).collect();
        let mask = [0x6d, 0xb6, 0xb2, 0x80];
        for offset in 0..8 {
            for len in 0..200 {
                let src = &data[offset..offset + len];
                let mut a = src.to_vec();
                mask_naive(&mut a, mask);
                let mut b = data.clone();
                mask_align_to(&mut b[offset..offset + len], mask);
                let mut c = src.to_vec();
                mask_chunks(&mut c, mask);
                assert_eq!(a, &b[offset..offset + len], "align_to off={offset} len={len}");
                assert_eq!(a, c, "chunks off={offset} len={len}");
            }
        }
    }

    #[test]
    fn mask_is_involution() {
        let mut v = b"Hello, actix!".to_vec();
        mask_align_to(&mut v, [1, 2, 3, 4]);
        mask_align_to(&mut v, [1, 2, 3, 4]);
        assert_eq!(v, b"Hello, actix!");
    }

    #[test]
    fn header_matches_checked() {
        for n in [0, 7, 42, 1_000_000, u64::MAX] {
            assert_eq!(header_from_u64(n), header_from_u64_checked(n));
        }
    }
}
