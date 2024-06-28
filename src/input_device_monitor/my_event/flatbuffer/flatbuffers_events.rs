use core::cmp::Ordering;
use core::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod coffee_time {

    use core::cmp::Ordering;
    use core::mem;

    extern crate flatbuffers;
    use self::flatbuffers::{EndianScalar, Follow};
    #[allow(unused_imports, dead_code)]
    pub mod input_events {

        use core::cmp::Ordering;
        use core::mem;

        extern crate flatbuffers;
        use self::flatbuffers::{EndianScalar, Follow};

        pub enum KeyboardOffset {}
        #[derive(Copy, Clone, PartialEq)]

        pub struct Keyboard<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for Keyboard<'a> {
            type Inner = Keyboard<'a>;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table::new(buf, loc),
                }
            }
        }

        impl<'a> Keyboard<'a> {
            pub const VT_TYPE_: flatbuffers::VOffsetT = 4;
            pub const VT_KEY: flatbuffers::VOffsetT = 6;

            #[inline]
            pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                Keyboard { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args KeyboardArgs<'args>,
            ) -> flatbuffers::WIPOffset<Keyboard<'bldr>> {
                let mut builder = KeyboardBuilder::new(_fbb);
                if let Some(x) = args.key {
                    builder.add_key(x);
                }
                if let Some(x) = args.type_ {
                    builder.add_type_(x);
                }
                builder.finish()
            }

            #[inline]
            pub fn type_(&self) -> Option<&'a str> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab
                        .get::<flatbuffers::ForwardsUOffset<&str>>(Keyboard::VT_TYPE_, None)
                }
            }
            #[inline]
            pub fn key(&self) -> Option<&'a str> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab
                        .get::<flatbuffers::ForwardsUOffset<&str>>(Keyboard::VT_KEY, None)
                }
            }
        }

        impl flatbuffers::Verifiable for Keyboard<'_> {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.visit_table(pos)?
                    .visit_field::<flatbuffers::ForwardsUOffset<&str>>(
                        "type_",
                        Self::VT_TYPE_,
                        false,
                    )?
                    .visit_field::<flatbuffers::ForwardsUOffset<&str>>("key", Self::VT_KEY, false)?
                    .finish();
                Ok(())
            }
        }
        pub struct KeyboardArgs<'a> {
            pub type_: Option<flatbuffers::WIPOffset<&'a str>>,
            pub key: Option<flatbuffers::WIPOffset<&'a str>>,
        }
        impl<'a> Default for KeyboardArgs<'a> {
            #[inline]
            fn default() -> Self {
                KeyboardArgs {
                    type_: None,
                    key: None,
                }
            }
        }

        pub struct KeyboardBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> KeyboardBuilder<'a, 'b> {
            #[inline]
            pub fn add_type_(&mut self, type_: flatbuffers::WIPOffset<&'b str>) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(Keyboard::VT_TYPE_, type_);
            }
            #[inline]
            pub fn add_key(&mut self, key: flatbuffers::WIPOffset<&'b str>) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(Keyboard::VT_KEY, key);
            }
            #[inline]
            pub fn new(
                _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            ) -> KeyboardBuilder<'a, 'b> {
                let start = _fbb.start_table();
                KeyboardBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<Keyboard<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        impl core::fmt::Debug for Keyboard<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut ds = f.debug_struct("Keyboard");
                ds.field("type_", &self.type_());
                ds.field("key", &self.key());
                ds.finish()
            }
        }
        pub enum PositionOffset {}
        #[derive(Copy, Clone, PartialEq)]

        pub struct Position<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for Position<'a> {
            type Inner = Position<'a>;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table::new(buf, loc),
                }
            }
        }

        impl<'a> Position<'a> {
            pub const VT_X: flatbuffers::VOffsetT = 4;
            pub const VT_Y: flatbuffers::VOffsetT = 6;

            #[inline]
            pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                Position { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args PositionArgs,
            ) -> flatbuffers::WIPOffset<Position<'bldr>> {
                let mut builder = PositionBuilder::new(_fbb);
                builder.add_y(args.y);
                builder.add_x(args.x);
                builder.finish()
            }

            #[inline]
            pub fn x(&self) -> f32 {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe { self._tab.get::<f32>(Position::VT_X, Some(0.0)).unwrap() }
            }
            #[inline]
            pub fn y(&self) -> f32 {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe { self._tab.get::<f32>(Position::VT_Y, Some(0.0)).unwrap() }
            }
        }

        impl flatbuffers::Verifiable for Position<'_> {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.visit_table(pos)?
                    .visit_field::<f32>("x", Self::VT_X, false)?
                    .visit_field::<f32>("y", Self::VT_Y, false)?
                    .finish();
                Ok(())
            }
        }
        pub struct PositionArgs {
            pub x: f32,
            pub y: f32,
        }
        impl<'a> Default for PositionArgs {
            #[inline]
            fn default() -> Self {
                PositionArgs { x: 0.0, y: 0.0 }
            }
        }

        pub struct PositionBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> PositionBuilder<'a, 'b> {
            #[inline]
            pub fn add_x(&mut self, x: f32) {
                self.fbb_.push_slot::<f32>(Position::VT_X, x, 0.0);
            }
            #[inline]
            pub fn add_y(&mut self, y: f32) {
                self.fbb_.push_slot::<f32>(Position::VT_Y, y, 0.0);
            }
            #[inline]
            pub fn new(
                _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            ) -> PositionBuilder<'a, 'b> {
                let start = _fbb.start_table();
                PositionBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<Position<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        impl core::fmt::Debug for Position<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut ds = f.debug_struct("Position");
                ds.field("x", &self.x());
                ds.field("y", &self.y());
                ds.finish()
            }
        }
        pub enum MouseOffset {}
        #[derive(Copy, Clone, PartialEq)]

        pub struct Mouse<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for Mouse<'a> {
            type Inner = Mouse<'a>;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table::new(buf, loc),
                }
            }
        }

        impl<'a> Mouse<'a> {
            pub const VT_TYPE_: flatbuffers::VOffsetT = 4;
            pub const VT_BUTTON: flatbuffers::VOffsetT = 6;
            pub const VT_POSITION: flatbuffers::VOffsetT = 8;

            #[inline]
            pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                Mouse { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args MouseArgs<'args>,
            ) -> flatbuffers::WIPOffset<Mouse<'bldr>> {
                let mut builder = MouseBuilder::new(_fbb);
                if let Some(x) = args.position {
                    builder.add_position(x);
                }
                if let Some(x) = args.button {
                    builder.add_button(x);
                }
                if let Some(x) = args.type_ {
                    builder.add_type_(x);
                }
                builder.finish()
            }

            #[inline]
            pub fn type_(&self) -> Option<&'a str> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab
                        .get::<flatbuffers::ForwardsUOffset<&str>>(Mouse::VT_TYPE_, None)
                }
            }
            #[inline]
            pub fn button(&self) -> Option<&'a str> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab
                        .get::<flatbuffers::ForwardsUOffset<&str>>(Mouse::VT_BUTTON, None)
                }
            }
            #[inline]
            pub fn position(&self) -> Option<Position<'a>> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab
                        .get::<flatbuffers::ForwardsUOffset<Position>>(Mouse::VT_POSITION, None)
                }
            }
        }

        impl flatbuffers::Verifiable for Mouse<'_> {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.visit_table(pos)?
                    .visit_field::<flatbuffers::ForwardsUOffset<&str>>(
                        "type_",
                        Self::VT_TYPE_,
                        false,
                    )?
                    .visit_field::<flatbuffers::ForwardsUOffset<&str>>(
                        "button",
                        Self::VT_BUTTON,
                        false,
                    )?
                    .visit_field::<flatbuffers::ForwardsUOffset<Position>>(
                        "position",
                        Self::VT_POSITION,
                        false,
                    )?
                    .finish();
                Ok(())
            }
        }
        pub struct MouseArgs<'a> {
            pub type_: Option<flatbuffers::WIPOffset<&'a str>>,
            pub button: Option<flatbuffers::WIPOffset<&'a str>>,
            pub position: Option<flatbuffers::WIPOffset<Position<'a>>>,
        }
        impl<'a> Default for MouseArgs<'a> {
            #[inline]
            fn default() -> Self {
                MouseArgs {
                    type_: None,
                    button: None,
                    position: None,
                }
            }
        }

        pub struct MouseBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> MouseBuilder<'a, 'b> {
            #[inline]
            pub fn add_type_(&mut self, type_: flatbuffers::WIPOffset<&'b str>) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(Mouse::VT_TYPE_, type_);
            }
            #[inline]
            pub fn add_button(&mut self, button: flatbuffers::WIPOffset<&'b str>) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(Mouse::VT_BUTTON, button);
            }
            #[inline]
            pub fn add_position(&mut self, position: flatbuffers::WIPOffset<Position<'b>>) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<Position>>(
                        Mouse::VT_POSITION,
                        position,
                    );
            }
            #[inline]
            pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MouseBuilder<'a, 'b> {
                let start = _fbb.start_table();
                MouseBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<Mouse<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        impl core::fmt::Debug for Mouse<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut ds = f.debug_struct("Mouse");
                ds.field("type_", &self.type_());
                ds.field("button", &self.button());
                ds.field("position", &self.position());
                ds.finish()
            }
        }
        pub enum EventOffset {}
        #[derive(Copy, Clone, PartialEq)]

        pub struct Event<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for Event<'a> {
            type Inner = Event<'a>;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table::new(buf, loc),
                }
            }
        }

        impl<'a> Event<'a> {
            pub const VT_KEYBOARD: flatbuffers::VOffsetT = 4;
            pub const VT_MOUSE: flatbuffers::VOffsetT = 6;

            #[inline]
            pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                Event { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args EventArgs<'args>,
            ) -> flatbuffers::WIPOffset<Event<'bldr>> {
                let mut builder = EventBuilder::new(_fbb);
                if let Some(x) = args.mouse {
                    builder.add_mouse(x);
                }
                if let Some(x) = args.keyboard {
                    builder.add_keyboard(x);
                }
                builder.finish()
            }

            #[inline]
            pub fn keyboard(&self) -> Option<Keyboard<'a>> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab
                        .get::<flatbuffers::ForwardsUOffset<Keyboard>>(Event::VT_KEYBOARD, None)
                }
            }
            #[inline]
            pub fn mouse(&self) -> Option<Mouse<'a>> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab
                        .get::<flatbuffers::ForwardsUOffset<Mouse>>(Event::VT_MOUSE, None)
                }
            }
        }

        impl flatbuffers::Verifiable for Event<'_> {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.visit_table(pos)?
                    .visit_field::<flatbuffers::ForwardsUOffset<Keyboard>>(
                        "keyboard",
                        Self::VT_KEYBOARD,
                        false,
                    )?
                    .visit_field::<flatbuffers::ForwardsUOffset<Mouse>>(
                        "mouse",
                        Self::VT_MOUSE,
                        false,
                    )?
                    .finish();
                Ok(())
            }
        }
        pub struct EventArgs<'a> {
            pub keyboard: Option<flatbuffers::WIPOffset<Keyboard<'a>>>,
            pub mouse: Option<flatbuffers::WIPOffset<Mouse<'a>>>,
        }
        impl<'a> Default for EventArgs<'a> {
            #[inline]
            fn default() -> Self {
                EventArgs {
                    keyboard: None,
                    mouse: None,
                }
            }
        }

        pub struct EventBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> EventBuilder<'a, 'b> {
            #[inline]
            pub fn add_keyboard(&mut self, keyboard: flatbuffers::WIPOffset<Keyboard<'b>>) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<Keyboard>>(
                        Event::VT_KEYBOARD,
                        keyboard,
                    );
            }
            #[inline]
            pub fn add_mouse(&mut self, mouse: flatbuffers::WIPOffset<Mouse<'b>>) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<Mouse>>(Event::VT_MOUSE, mouse);
            }
            #[inline]
            pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> EventBuilder<'a, 'b> {
                let start = _fbb.start_table();
                EventBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<Event<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        impl core::fmt::Debug for Event<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut ds = f.debug_struct("Event");
                ds.field("keyboard", &self.keyboard());
                ds.field("mouse", &self.mouse());
                ds.finish()
            }
        }
        #[inline]
        /// Verifies that a buffer of bytes contains a `Event`
        /// and returns it.
        /// Note that verification is still experimental and may not
        /// catch every error, or be maximally performant. For the
        /// previous, unchecked, behavior use
        /// `root_as_event_unchecked`.
        pub fn root_as_event(buf: &[u8]) -> Result<Event, flatbuffers::InvalidFlatbuffer> {
            flatbuffers::root::<Event>(buf)
        }
        #[inline]
        /// Verifies that a buffer of bytes contains a size prefixed
        /// `Event` and returns it.
        /// Note that verification is still experimental and may not
        /// catch every error, or be maximally performant. For the
        /// previous, unchecked, behavior use
        /// `size_prefixed_root_as_event_unchecked`.
        pub fn size_prefixed_root_as_event(
            buf: &[u8],
        ) -> Result<Event, flatbuffers::InvalidFlatbuffer> {
            flatbuffers::size_prefixed_root::<Event>(buf)
        }
        #[inline]
        /// Verifies, with the given options, that a buffer of bytes
        /// contains a `Event` and returns it.
        /// Note that verification is still experimental and may not
        /// catch every error, or be maximally performant. For the
        /// previous, unchecked, behavior use
        /// `root_as_event_unchecked`.
        pub fn root_as_event_with_opts<'b, 'o>(
            opts: &'o flatbuffers::VerifierOptions,
            buf: &'b [u8],
        ) -> Result<Event<'b>, flatbuffers::InvalidFlatbuffer> {
            flatbuffers::root_with_opts::<Event<'b>>(opts, buf)
        }
        #[inline]
        /// Verifies, with the given verifier options, that a buffer of
        /// bytes contains a size prefixed `Event` and returns
        /// it. Note that verification is still experimental and may not
        /// catch every error, or be maximally performant. For the
        /// previous, unchecked, behavior use
        /// `root_as_event_unchecked`.
        pub fn size_prefixed_root_as_event_with_opts<'b, 'o>(
            opts: &'o flatbuffers::VerifierOptions,
            buf: &'b [u8],
        ) -> Result<Event<'b>, flatbuffers::InvalidFlatbuffer> {
            flatbuffers::size_prefixed_root_with_opts::<Event<'b>>(opts, buf)
        }
        #[inline]
        /// Assumes, without verification, that a buffer of bytes contains a Event and returns it.
        /// # Safety
        /// Callers must trust the given bytes do indeed contain a valid `Event`.
        pub unsafe fn root_as_event_unchecked(buf: &[u8]) -> Event {
            flatbuffers::root_unchecked::<Event>(buf)
        }
        #[inline]
        /// Assumes, without verification, that a buffer of bytes contains a size prefixed Event and returns it.
        /// # Safety
        /// Callers must trust the given bytes do indeed contain a valid size prefixed `Event`.
        pub unsafe fn size_prefixed_root_as_event_unchecked(buf: &[u8]) -> Event {
            flatbuffers::size_prefixed_root_unchecked::<Event>(buf)
        }
        #[inline]
        pub fn finish_event_buffer<'a, 'b>(
            fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            root: flatbuffers::WIPOffset<Event<'a>>,
        ) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_event_buffer<'a, 'b>(
            fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            root: flatbuffers::WIPOffset<Event<'a>>,
        ) {
            fbb.finish_size_prefixed(root, None);
        }
    } // pub mod InputEvents
} // pub mod CoffeeTime
