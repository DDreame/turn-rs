use super::{Message, MAGIC_COOKIE, util};
use bytes::{BufMut, BytesMut};

/// 编码消息
///
/// 将消息结构编码为缓冲区.
pub fn encoder(message: Message) -> BytesMut {
    let mut attributes = BytesMut::new();
    let mut buffer = BytesMut::new();

    // 遍历所有属性值,
    // 将所有属性值转换为缓冲区.
    for (k, v) in message.attributes {
        let value = v.into_bytes(message.transaction);

        // 值长度
        // 值填充长度
        let size = value.len();
        let psize = util::pad_size(size);

        // 属性类型
        // 属性值长度
        // 属性值
        attributes.put_u16(k as u16);
        attributes.put_u16(size as u16);
        attributes.put(&value[..]);

        // 如果需要填充，
        // 则填充指定位0.
        if psize > 0 {
            let pad = vec![0u8; psize];
            attributes.put(&pad[..]);
        }
    }

    // 消息类型
    // 消息长度
    // 固定Cookie
    // 交易号
    // 属性列表
    buffer.put_u16(message.flag as u16);
    buffer.put_u16(attributes.len() as u16);
    buffer.put_u32(MAGIC_COOKIE);
    buffer.put(&message.transaction[..]);
    buffer.put(attributes);
    buffer
}
