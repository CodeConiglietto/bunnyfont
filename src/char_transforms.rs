#[derive(Clone, Copy, Debug)]
pub enum CharRotation {
    None,
    Rotation90,
    Rotation180,
    Rotation270,
}

impl CharRotation {
    pub fn then(self, rhs: Self) -> Self {
        match (self, rhs) {
            (CharRotation::None, CharRotation::None)
            | (CharRotation::Rotation90, CharRotation::Rotation270)
            | (CharRotation::Rotation180, CharRotation::Rotation180)
            | (CharRotation::Rotation270, CharRotation::Rotation90) => CharRotation::None,

            (CharRotation::None, CharRotation::Rotation90)
            | (CharRotation::Rotation90, CharRotation::None)
            | (CharRotation::Rotation180, CharRotation::Rotation270)
            | (CharRotation::Rotation270, CharRotation::Rotation180) => CharRotation::Rotation90,

            (CharRotation::None, CharRotation::Rotation180)
            | (CharRotation::Rotation90, CharRotation::Rotation90)
            | (CharRotation::Rotation180, CharRotation::None)
            | (CharRotation::Rotation270, CharRotation::Rotation270) => CharRotation::Rotation180,

            (CharRotation::None, CharRotation::Rotation270)
            | (CharRotation::Rotation90, CharRotation::Rotation180)
            | (CharRotation::Rotation180, CharRotation::Rotation90)
            | (CharRotation::Rotation270, CharRotation::None) => CharRotation::Rotation270,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CharMirror {
    None,
    MirrorX,
    MirrorY,
    MirrorBoth,
}

impl CharMirror {
    pub fn then(self, rhs: Self) -> Self {
        match (self, rhs) {
            (CharMirror::None, CharMirror::None)
            | (CharMirror::MirrorX, CharMirror::MirrorX)
            | (CharMirror::MirrorY, CharMirror::MirrorY)
            | (CharMirror::MirrorBoth, CharMirror::MirrorBoth) => CharMirror::None,

            (CharMirror::None, CharMirror::MirrorX)
            | (CharMirror::MirrorX, CharMirror::None)
            | (CharMirror::MirrorY, CharMirror::MirrorBoth)
            | (CharMirror::MirrorBoth, CharMirror::MirrorY) => CharMirror::MirrorX,

            (CharMirror::None, CharMirror::MirrorY)
            | (CharMirror::MirrorX, CharMirror::MirrorBoth)
            | (CharMirror::MirrorY, CharMirror::None)
            | (CharMirror::MirrorBoth, CharMirror::MirrorX) => CharMirror::MirrorY,

            (CharMirror::None, CharMirror::MirrorBoth)
            | (CharMirror::MirrorX, CharMirror::MirrorY)
            | (CharMirror::MirrorY, CharMirror::MirrorX)
            | (CharMirror::MirrorBoth, CharMirror::None) => CharMirror::MirrorBoth,
        }
    }
}
