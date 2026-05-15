
    pub const DDRD: *mut u8 = 0x2A as *mut u8; // PORT D DIRECTION REGISTER
    pub const PORTD: *mut u8 = 0x2B as *mut u8; // PORT D OUTPUT DATA REGISTER
    pub const TIFR0: *mut u8 = 0x35 as *mut u8; // TIMER COUNTER 0 INTERRUPT FLAG REGISTER
    pub const OCR0A: *mut u8 = 0x47 as *mut u8; // OUTPUT COMPARE REIGSTER A
    pub const OC0AS_BIT: u8 = 1 << 5;
    pub const TCCR0A: *mut u8 = 0x44 as *mut u8; // TC0 CONTROL REGISTER A
    pub const COM0B1_BIT: u8 = 1 << 5;
    pub const WGM00_BIT: u8 = 1 << 0;
    pub const WGM01_BIT: u8 = 1 << 1;
    pub const TCCR0B: *mut u8 = 0x45 as *mut u8; // TC0 CONTROL REGISTER B
    pub const WGM02_BIT: u8 = 1 << 3;
    pub const OCR0B: *mut u8 = 0x48 as *mut u8; // TC0 Output compare register B
    pub const PIN5: u8 = 1 << 5;
    pub const PIN6: u8 = 1 << 6;
    pub const TOV0_BIT: u8 = 1 << 0;
    pub const CS00_BIT: u8 = 1 << 0;
    pub const TCNT0: *mut u8 = 0x46 as *mut u8; // TC0 count value register
