use std::{fmt::Display, sync::Arc};

use epaint::{ColorImage, Pos2, Vec2};

use crate::{Context, Id};

/// This is used to send a command to a specific viewport
///
/// This is returned by `Context::get_viewport_id` and `Context::get_parent_viewport_id`
#[derive(Default, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct ViewportId(pub(crate) u64);

impl Display for ViewportId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ViewportId {
    /// This will return the `ViewportId` of the main viewport
    pub const MAIN: Self = Self(0);
}

/// This will deref to `ViewportIdPair::this`
#[derive(Default, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct ViewportIdPair {
    pub this: ViewportId,
    pub parent: ViewportId,
}

impl ViewportIdPair {
    /// This will return the `ViewportIdPair` of the main viewport
    pub const MAIN: Self = Self {
        this: ViewportId::MAIN,
        parent: ViewportId::MAIN,
    };

    pub fn new(this: ViewportId, parent: ViewportId) -> Self {
        Self { this, parent }
    }
}

impl std::ops::Deref for ViewportIdPair {
    type Target = ViewportId;

    fn deref(&self) -> &Self::Target {
        &self.this
    }
}

/// This is used to render an async viewport
pub type ViewportRender = dyn Fn(&Context) + Sync + Send;

pub type ViewportRenderSyncCallback =
    dyn for<'a> Fn(&Context, ViewportBuilder, ViewportIdPair, Box<dyn FnOnce(&Context) + 'a>);

/// The filds in this struct should not be change directly, but is not problem tho!
/// Every thing is wrapped in ``Option<T>`` indicates that nothing changed from the last ``ViewportBuilder``!
#[derive(PartialEq, Eq, Clone)]
#[allow(clippy::option_option)]
pub struct ViewportBuilder {
    pub id: Id,
    pub title: String,
    pub name: Option<(String, String)>,
    pub position: Option<Option<Pos2>>,
    pub inner_size: Option<Option<Vec2>>,
    pub fullscreen: Option<bool>,
    pub maximized: Option<bool>,
    pub resizable: Option<bool>,
    pub transparent: Option<bool>,
    pub decorations: Option<bool>,
    pub icon: Option<Option<Arc<ColorImage>>>,
    pub active: Option<bool>,
    pub visible: Option<bool>,
    pub title_hidden: Option<bool>,
    pub titlebar_transparent: Option<bool>,
    pub fullsize_content_view: Option<bool>,
    pub min_inner_size: Option<Option<Vec2>>,
    pub max_inner_size: Option<Option<Vec2>>,
    pub drag_and_drop: Option<bool>,

    pub close_button: Option<bool>,
    pub minimize_button: Option<bool>,
    pub maximize_button: Option<bool>,

    pub hittest: Option<bool>,
}

impl ViewportBuilder {
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            title: "Dummy egui viewport".into(),
            name: None,
            position: None,
            inner_size: Some(Some(Vec2::new(300.0, 200.0))),
            fullscreen: None,
            maximized: None,
            resizable: Some(true),
            transparent: Some(true),
            decorations: Some(true),
            icon: None,
            active: Some(true),
            visible: Some(true),
            title_hidden: None,
            titlebar_transparent: None,
            fullsize_content_view: None,
            min_inner_size: None,
            max_inner_size: None,
            drag_and_drop: Some(true),
            close_button: Some(true),
            minimize_button: Some(true),
            maximize_button: Some(true),
            hittest: Some(true),
        }
    }
}

impl ViewportBuilder {
    pub fn empty(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            title: "Dummy egui viewport".into(),
            name: None,
            position: None,
            inner_size: None,
            fullscreen: None,
            maximized: None,
            resizable: None,
            transparent: None,
            decorations: None,
            icon: None,
            active: None,
            visible: None,
            title_hidden: None,
            titlebar_transparent: None,
            fullsize_content_view: None,
            min_inner_size: None,
            max_inner_size: None,
            drag_and_drop: None,
            close_button: None,
            minimize_button: None,
            maximize_button: None,
            hittest: None,
        }
    }

    /// Sets the initial title of the window in the title bar.
    ///
    /// The default is `"Dummy egui viewport"`.
    ///
    /// Look at winit for more details
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets whether the window should have a border, a title bar, etc.
    ///
    /// The default is `true`.
    ///
    /// Look at winit for more details
    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = Some(decorations);
        self
    }

    /// Sets whether the window should be put into fullscreen upon creation.
    ///
    /// The default is `None`.
    ///
    /// Look at winit for more details
    /// This will use borderless
    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = Some(fullscreen);
        self
    }

    /// Request that the window is maximized upon creation.
    ///
    /// The default is `false`.
    ///
    /// Look at winit for more details
    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.maximized = Some(maximized);
        self
    }

    /// Sets whether the window is resizable or not.
    ///
    /// The default is `true`.
    ///
    /// Look at winit for more details
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    /// Sets whether the background of the window should be transparent.
    ///
    /// If this is `true`, writing colors with alpha values different than
    /// `1.0` will produce a transparent window. On some platforms this
    /// is more of a hint for the system and you'd still have the alpha
    /// buffer.
    ///
    /// The default is `false`.
    /// If this is not working is because the graphic context dozen't support transparency,
    /// you will need to set the transparency in the eframe!
    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.transparent = Some(transparent);
        self
    }

    /// The icon needs to be wrapped in Arc because will be cloned every frame
    pub fn with_window_icon(mut self, icon: Option<Arc<ColorImage>>) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Whether the window will be initially focused or not.
    ///
    /// The window should be assumed as not focused by default
    ///
    /// ## Platform-specific:
    ///
    /// **Android / iOS / X11 / Wayland / Orbital:** Unsupported.
    ///
    /// Look at winit for more details
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    /// Sets whether the window will be initially visible or hidden.
    ///
    /// The default is to show the window.
    ///
    /// Look at winit for more details
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    /// Mac Os only
    /// Hides the window title.
    pub fn with_title_hidden(mut self, title_hidden: bool) -> Self {
        self.title_hidden = Some(title_hidden);
        self
    }

    /// Mac Os only
    /// Makes the titlebar transparent and allows the content to appear behind it.
    pub fn with_titlebar_transparent(mut self, value: bool) -> Self {
        self.titlebar_transparent = Some(value);
        self
    }

    /// Mac Os only
    /// Makes the window content appear behind the titlebar.
    pub fn with_fullsize_content_view(mut self, value: bool) -> Self {
        self.fullsize_content_view = Some(value);
        self
    }

    /// Requests the window to be of specific dimensions.
    ///
    /// If this is not set, some platform-specific dimensions will be used.
    ///
    /// Should be bigger then 0
    /// Look at winit for more details
    pub fn with_inner_size(mut self, value: Option<Vec2>) -> Self {
        self.inner_size = Some(value);
        self
    }

    /// Sets the minimum dimensions a window can have.
    ///
    /// If this is not set, the window will have no minimum dimensions (aside
    /// from reserved).
    ///
    /// Should be bigger then 0
    /// Look at winit for more details
    pub fn with_min_inner_size(mut self, value: Option<Vec2>) -> Self {
        self.min_inner_size = Some(value);
        self
    }

    /// Sets the maximum dimensions a window can have.
    ///
    /// If this is not set, the window will have no maximum or will be set to
    /// the primary monitor's dimensions by the platform.
    ///
    /// Should be bigger then 0
    /// Look at winit for more details
    pub fn with_max_inner_size(mut self, value: Option<Vec2>) -> Self {
        self.max_inner_size = Some(value);
        self
    }

    /// X11 not working!
    pub fn with_close_button(mut self, value: bool) -> Self {
        self.close_button = Some(value);
        self
    }

    /// X11 not working!
    pub fn with_minimize_button(mut self, value: bool) -> Self {
        self.minimize_button = Some(value);
        self
    }

    /// X11 not working!
    pub fn with_maximize_button(mut self, value: bool) -> Self {
        self.maximize_button = Some(value);
        self
    }

    /// This currently only work on windows to be disabled!
    pub fn with_drag_and_drop(mut self, value: bool) -> Self {
        self.drag_and_drop = Some(value);
        self
    }

    pub fn with_position(mut self, value: Option<Pos2>) -> Self {
        self.position = Some(value);
        self
    }

    /// This is wayland only!
    /// Build window with the given name.
    ///
    /// The `general` name sets an application ID, which should match the `.desktop`
    /// file distributed with your program. The `instance` is a `no-op`.
    ///
    /// For details about application ID conventions, see the
    /// [Desktop Entry Spec](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#desktop-file-id)
    pub fn with_name(mut self, id: impl Into<String>, instance: impl Into<String>) -> Self {
        self.name = Some((id.into(), instance.into()));
        self
    }

    /// Is not implemented for winit
    /// You should use `ViewportCommand::CursorHitTest` if you want to set this!
    #[deprecated]
    pub fn with_hittest(mut self, value: bool) -> Self {
        self.hittest = Some(value);
        self
    }
}

/// You can send a `ViewportCommand` to the viewport with `Context::viewport_command`
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ViewportCommand {
    Title(String),
    Transparent(bool),
    Visible(bool),
    Drag,
    OuterPosition(Pos2),

    /// Should be bigger then 0
    InnerSize(Vec2),

    /// Should be bigger then 0
    MinInnerSize(Option<Vec2>),

    /// Should be bigger then 0
    MaxInnerSize(Option<Vec2>),
    ResizeIncrements(Option<Vec2>),

    /// Top, Bottom, Right, Left
    Resize(bool, bool, bool, bool),
    Resizable(bool),
    EnableButtons {
        close: bool,
        minimized: bool,
        maximize: bool,
    },
    Minimized(bool),
    Maximized(bool),
    Fullscreen(bool),
    Decorations(bool),

    /// 0 = Normal, 1 = AlwaysOnBottom, 2 = AlwaysOnTop
    WindowLevel(u8),
    WindowIcon(Option<ColorImage>),
    IMEPosition(Pos2),
    IMEAllowed(bool),

    /// 0 = Normal, 1 = Password, 2 = Terminal
    IMEPurpose(u8),

    /// 0 = Informational, 1 = Critical
    RequestUserAttention(Option<u8>),

    /// 0 = Light, 1 = Dark
    SetTheme(Option<u8>),

    ContentProtected(bool),

    CursorPosition(Pos2),

    /// 0 = None, 1 = Confined, 2 = Locked
    CursorGrab(u8),

    CursorVisible(bool),

    CursorHitTest(bool),
}

#[derive(Clone)]
pub(crate) struct Viewport {
    pub(crate) builder: ViewportBuilder,
    pub(crate) pair: ViewportIdPair,
    pub(crate) used: bool,
    pub(crate) render: Option<Arc<Box<ViewportRender>>>,
}

#[derive(Clone)]
pub struct ViewportOutput {
    pub builder: ViewportBuilder,
    pub pair: ViewportIdPair,
    pub render: Option<Arc<Box<ViewportRender>>>,
}