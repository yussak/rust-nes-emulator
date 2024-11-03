pub struct CPU {
    // 符号なし8bitなので2^8=256 0~255までを表現
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    // CPUインスタンスを作成し、構造体の各フィールドの初期値を設定
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        // x=x|yは、x,yの各ビットが1なら1に書き換え、それをxとしている
        // 例）x = 0b0000_0000 | 0b0000_0010 => x = 0b0000_0010
        if result == 0 {
            // statusの特定のビットを1にセットしてresultがゼロであることを記録する　右から2番目が1なのでresultが0だと判別できる
            self.status = self.status | 0b0000_0010;
        } else {
            // 右から２番目以外を1にしているので上記と区別できているんだな
            // &なので各ビットが両方とも1なら1に変える
            // 例) x = 0b1010_1011 & 0b1111_1101 => x = 0b1010_1001 1でも0に変えるんだな
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    // selfはCPUインスタンス
    // Vec: サイズ変更可能な配列で中身がu8
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            // program配列には数値が入るが、これがコマンドになる
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                // この条件のときにこれを実行という割当をこの部分に書いていく
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.lda(param);
                }
                0xAA => {
                    self.tax();
                }
                0xe8 => {
                    self.inx();
                }
                0x00 => {
                    return;
                }

                // 上記のどれにも当てはまらない場合の処理
                _ => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);

        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);

        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
