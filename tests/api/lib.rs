#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use env::wire::Span as _;
    use env::{PieToken, TaskId};
    use terminal_api::frame as frame;
    use terminal_api::frame as terminal;
    use wire::message::Message;

    fn token(value: u64) -> PieToken {
        PieToken::from_bytes(&value.to_le_bytes()).unwrap()
    }

    #[test]
    fn terminal_command_and_reply_keep_their_fixed_layouts() {
        let command = terminal::Command {
            op: terminal::ATTACH,
            task: TaskId::new(0x0807_0605_0403_0201),
            authority: token(0x1817_1615_1413_1211),
            back: token(0x2827_2625_2423_2221),
        };
        assert_eq!(terminal::Command::LEN, 25);
        let mut bytes = [0; terminal::Command::LEN];
        let n = command.store_at(&mut bytes, 0).unwrap();
        assert_eq!(n, bytes.len());
        assert_eq!(
            &bytes,
            &[
                1, 1, 2, 3, 4, 5, 6, 7, 8, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x21,
                0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
            ]
        );
        let (decoded, end) = terminal::Command::fetch_at(&bytes, 0).unwrap();
        assert_eq!(end, n);
        assert_eq!(decoded.op, command.op);
        assert_eq!(decoded.task, command.task);
        assert_eq!(decoded.authority, command.authority);
        assert_eq!(decoded.back, command.back);

        let reply = terminal::Reply {
            status: 0,
            authority: token(0x0807_0605_0403_0201),
            input: token(9), output: token(10), control: token(11),
        };
        assert_eq!(terminal::Reply::LEN, 33);
        let mut bytes = [0; terminal::Reply::LEN];
        let n = reply.store_at(&mut bytes, 0).unwrap();
        assert_eq!(&bytes[..9], &[0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(&bytes[9..17], &9u64.to_le_bytes());
        assert_eq!(&bytes[17..25], &10u64.to_le_bytes());
        assert_eq!(&bytes[25..33], &11u64.to_le_bytes());
        let (decoded, end) = terminal::Reply::fetch_at(&bytes[..n], 0).unwrap();
        assert_eq!(end, n);
        assert_eq!(decoded.status, reply.status);
        assert_eq!(decoded.authority, reply.authority);
        assert_eq!(decoded.input, reply.input);
        assert_eq!(decoded.output, reply.output);
        assert_eq!(decoded.control, reply.control);
        for end in 0..terminal::Reply::LEN {
            assert!(terminal::Reply::fetch_at(&bytes[..end], 0).is_none());
        }
    }

    #[test]
    fn terminal_input_encodes_only_its_payload_and_rejects_short_or_long_frames() {
        let input = terminal::Input::data(&[0xaa, 0xbb, 0xcc]).unwrap();
        let mut bytes = terminal::Input::EMPTY;
        let n = input.store(&mut bytes).unwrap();
        assert_eq!(&bytes[..n], &[terminal::DATA, 3, 0, 0, 0, 0xaa, 0xbb, 0xcc]);
        let decoded = terminal::Input::fetch(&bytes[..n]).unwrap();
        assert_eq!(decoded.kind, terminal::DATA);
        assert_eq!(decoded.bytes(), input.bytes());
        assert!(terminal::Input::fetch(&bytes[..n - 1]).is_none());
        assert!(terminal::Input::fetch(&bytes[..n + 1]).is_none());

        let eof = terminal::Input::eof();
        let n = eof.store(&mut bytes).unwrap();
        assert_eq!(&bytes[..n], &[terminal::EOF, 0, 0, 0, 0]);
        let decoded = terminal::Input::fetch(&bytes[..n]).unwrap();
        assert_eq!(decoded.kind, eof.kind);
        assert!(decoded.bytes().is_empty());
        assert!(terminal::Input::data(&[]).is_none());
        assert!(terminal::Input::data(&[0; terminal::MAX + 1]).is_none());
    }

    #[test]
    fn eof_is_distinct_from_a_line_and_preserves_binary_data() {
        let line = frame::Input::data(b"a\0b\n").unwrap();
        let mut bytes = [0; frame::Input::LEN];
        let n = line.store(&mut bytes).unwrap();
        let decoded = frame::Input::fetch(&bytes[..n]).unwrap();
        assert_eq!(decoded.kind, frame::DATA);
        assert_eq!(decoded.bytes(), b"a\0b\n");
        let n = frame::Input::eof().store(&mut bytes).unwrap();
        let decoded = frame::Input::fetch(&bytes[..n]).unwrap();
        assert_eq!(decoded.kind, frame::EOF);
        assert!(decoded.bytes().is_empty());
        assert!(frame::Input::data(&[]).is_none());
    }
    #[test]
    fn malformed_or_truncated_input_frames_are_rejected() {
        let mut bytes = [0; frame::Input::LEN];
        let n = frame::Input::data(b"\n").unwrap().store(&mut bytes).unwrap();
        assert!(frame::Input::fetch(&bytes[..n - 1]).is_none());
        bytes[0] = frame::EOF;
        assert!(frame::Input::fetch(&bytes[..n]).is_none());
        bytes[0] = 255;
        assert!(frame::Input::fetch(&bytes[..n]).is_none());
        bytes[0] = frame::DATA;
        bytes[1..5].copy_from_slice(&((frame::MAX + 1) as u32).to_le_bytes());
        assert!(frame::Input::fetch(&bytes).is_none());
    }
    #[test]
    fn input_size_is_bounded_independently_of_uart_frames() {
        assert!(frame::Input::data(&[b'x'; frame::MAX]).is_some());
        assert!(frame::Input::data(&[b'x'; frame::MAX + 1]).is_none());
    }
}
