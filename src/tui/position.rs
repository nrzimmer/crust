macro_rules! impl_from_for_struct_inner_2 {
    ($struct_name:ident, $fa:ident, $fb:ident, $t:ty, $u:ty) => {
        impl From<($t, $u)> for $struct_name {
            fn from(value: ($t, $u)) -> Self {
                Self {
                    $fa: TryInto::<u16>::try_into(value.0).unwrap_or_else(|_| if value.0 > 0 { u16::MAX } else { u16::MIN }),
                    $fb: TryInto::<u16>::try_into(value.1).unwrap_or_else(|_| if value.1 > 0 { u16::MAX } else { u16::MIN }),
                }
            }
        }
    };
}

macro_rules! impl_from_for_struct_inner {
    ($struct_name:ident, $fa:ident, $fb:ident, $t:ty) => {
        impl_from_for_struct_inner_2!($struct_name, $fa, $fb, $t, u8);
        impl_from_for_struct_inner_2!($struct_name, $fa, $fb, $t, i8);
        impl_from_for_struct_inner_2!($struct_name, $fa, $fb, $t, i16);
        impl_from_for_struct_inner_2!($struct_name, $fa, $fb, $t, u16);
        impl_from_for_struct_inner_2!($struct_name, $fa, $fb, $t, u32);
        impl_from_for_struct_inner_2!($struct_name, $fa, $fb, $t, i32);
        impl_from_for_struct_inner_2!($struct_name, $fa, $fb, $t, u64);
        impl_from_for_struct_inner_2!($struct_name, $fa, $fb, $t, i64);
    };
}

macro_rules! impl_from_for_struct {
    ($struct_name:ident, $fa:ident, $fb:ident) => {
        impl_from_for_struct_inner!($struct_name, $fa, $fb, u8);
        impl_from_for_struct_inner!($struct_name, $fa, $fb, i8);
        impl_from_for_struct_inner!($struct_name, $fa, $fb, u16);
        impl_from_for_struct_inner!($struct_name, $fa, $fb, i16);
        impl_from_for_struct_inner!($struct_name, $fa, $fb, u32);
        impl_from_for_struct_inner!($struct_name, $fa, $fb, i32);
        impl_from_for_struct_inner!($struct_name, $fa, $fb, u64);
        impl_from_for_struct_inner!($struct_name, $fa, $fb, i64);
    };
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}
impl_from_for_struct!(Point, x, y);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}
impl_from_for_struct!(Size, width, height);

impl Point {
    pub fn set_x(&self, x: u16) -> Self {
        (x, self.y).into()
    }

    pub fn set_y(&self, y: u16) -> Self {
        (self.x, y).into()
    }
}

impl Size {
    pub fn set_width(&self, w: u16) -> Self {
        (w, self.height).into()
    }

    pub fn set_height(&self, h: u16) -> Self {
        (self.width, h).into()
    }
}
