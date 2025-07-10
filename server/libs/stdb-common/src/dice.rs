use spacetimedb::ReducerContext;

pub trait DiceExt {
    /// 1/6 (16.67%) chance
    fn random_d6(&self) -> u32;
    fn is_random_d6(&self) -> bool {
        self.random_d6() == 6
    }

    /// 1/8 (12.5%) chance
    fn random_d8(&self) -> u32;
    fn is_random_d8(&self) -> bool {
        self.random_d8() == 8
    }

    /// 1/16 (6.25%) chance
    fn random_d16(&self) -> u32;
    fn is_random_d16(&self) -> bool {
        self.random_d16() == 16
    }

    /// 1/32 (3.125%) chance
    fn random_d32(&self) -> u32;
    fn is_random_d32(&self) -> bool {
        self.random_d32() == 32
    }

    /// 1/128 (0.78%) chance
    fn random_d128(&self) -> u32;
    fn is_random_d128(&self) -> bool {
        self.random_d128() == 128
    }

    /// 1/1024 (0.0976%) chance
    fn random_d1024(&self) -> u32;
    fn is_random_d1024(&self) -> bool {
        self.random_d1024() == 1024
    }

    /// 1/16_384 (0.0061%) chance
    fn random_d16_384(&self) -> u32;
    fn is_random_d16_384(&self) -> bool {
        self.random_d16_384() == 16_384
    }
}

impl DiceExt for ReducerContext {
    fn random_d6(&self) -> u32 {
        self.random::<u32>() % 6 + 1
    }

    fn random_d8(&self) -> u32 {
        self.random::<u32>() % 8 + 1
    }

    fn random_d16(&self) -> u32 {
        self.random::<u32>() % 16 + 1
    }

    fn random_d32(&self) -> u32 {
        self.random::<u32>() % 32 + 1
    }

    fn random_d128(&self) -> u32 {
        self.random::<u32>() % 128 + 1
    }

    fn random_d1024(&self) -> u32 {
        self.random::<u32>() % 1024 + 1
    }

    fn random_d16_384(&self) -> u32 {
        self.random::<u32>() % 16_384 + 1
    }
}
