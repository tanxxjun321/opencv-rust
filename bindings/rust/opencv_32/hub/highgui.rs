#![allow(unused_parens)]
//! # High-level GUI
//! 
//! While OpenCV was designed for use in full-scale applications and can be used within functionally
//! rich UI frameworks (such as Qt\*, WinForms\*, or Cocoa\*) or without any UI at all, sometimes there
//! it is required to try functionality quickly and visualize the results. This is what the HighGUI
//! module has been designed for.
//! 
//! It provides easy interface to:
//! 
//! *   Create and manipulate windows that can display images and "remember" their content (no need to
//!    handle repaint events from OS).
//! *   Add trackbars to the windows, handle simple mouse events as well as keyboard commands.
//!    # OpenGL support
//!    # Qt New Functions
//! 
//!    ![image](https://docs.opencv.org/3.2.0/qtgui.png)
//! 
//!    This figure explains new functionality implemented with Qt\* GUI. The new GUI provides a statusbar,
//!    a toolbar, and a control panel. The control panel can have trackbars and buttonbars attached to it.
//!    If you cannot see the control panel, press Ctrl+P or right-click any Qt window and select **Display
//!    properties window**.
//! 
//!    *   To attach a trackbar, the window name parameter must be NULL.
//! 
//!    *   To attach a buttonbar, a button must be created. If the last bar attached to the control panel
//!        is a buttonbar, the new button is added to the right of the last button. If the last bar
//!        attached to the control panel is a trackbar, or the control panel is empty, a new buttonbar is
//!        created. Then, a new button is attached to it.
//! 
//!    See below the example used to generate the figure:
//!    ```ignore
//!        int main(int argc, char *argv[])
//!        {
//! 
//!            int value = 50;
//!            int value2 = 0;
//! 
//! 
//!            namedWindow("main1",WINDOW_NORMAL);
//!            namedWindow("main2",WINDOW_AUTOSIZE | CV_GUI_NORMAL);
//!            createTrackbar( "track1", "main1", &value, 255,  NULL);
//! 
//!            String nameb1 = "button1";
//!            String nameb2 = "button2";
//! 
//!            createButton(nameb1,callbackButton,&nameb1,QT_CHECKBOX,1);
//!            createButton(nameb2,callbackButton,NULL,QT_CHECKBOX,0);
//!            createTrackbar( "track2", NULL, &value2, 255, NULL);
//!            createButton("button5",callbackButton1,NULL,QT_RADIOBOX,0);
//!            createButton("button6",callbackButton2,NULL,QT_RADIOBOX,1);
//! 
//!            setMouseCallback( "main2",on_mouse,NULL );
//! 
//!            Mat img1 = imread("files/flower.jpg");
//!            VideoCapture video;
//!            video.open("files/hockey.avi");
//! 
//!            Mat img2,img3;
//! 
//!            while( waitKey(33) != 27 )
//!            {
//!                img1.convertTo(img2,-1,1,value);
//!                video >> img3;
//! 
//!                imshow("main1",img2);
//!                imshow("main2",img3);
//!            }
//! 
//!            destroyAllWindows();
//! 
//!            return 0;
//!        }
//!    ```
//! 
//! 
//! 
//!    # WinRT support
//! 
//!    This figure explains new functionality implemented with WinRT GUI. The new GUI provides an Image control,
//!    and a slider panel. Slider panel holds trackbars attached to it.
//! 
//!    Sliders are attached below the image control. Every new slider is added below the previous one.
//! 
//!    See below the example used to generate the figure:
//!    ```ignore
//!        void sample_app::MainPage::ShowWindow()
//!        {
//!            static cv::String windowName("sample");
//!            cv::winrt_initContainer(this->cvContainer);
//!            cv::namedWindow(windowName); // not required
//! 
//!            cv::Mat image = cv::imread("Assets/sample.jpg");
//!            cv::Mat converted = cv::Mat(image.rows, image.cols, CV_8UC4);
//!            cv::cvtColor(image, converted, COLOR_BGR2BGRA);
//!            cv::imshow(windowName, converted); // this will create window if it hasn't been created before
//! 
//!            int state = 42;
//!            cv::TrackbarCallback callback = [](int pos, void* userdata)
//!            {
//!                if (pos == 0) {
//!                    cv::destroyWindow(windowName);
//!                }
//!            };
//!            cv::TrackbarCallback callbackTwin = [](int pos, void* userdata)
//!            {
//!                if (pos >= 70) {
//!                    cv::destroyAllWindows();
//!                }
//!            };
//!            cv::createTrackbar("Sample trackbar", windowName, &state, 100, callback);
//!            cv::createTrackbar("Twin brother", windowName, &state, 100, callbackTwin);
//!        }
//!    ```
//! 
//! 
//!    # C API
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::QtFontTrait };
}

/// indicates that ALT Key is pressed.
pub const EVENT_FLAG_ALTKEY: i32 = 32;
/// indicates that CTRL Key is pressed.
pub const EVENT_FLAG_CTRLKEY: i32 = 8;
/// indicates that the left mouse button is down.
pub const EVENT_FLAG_LBUTTON: i32 = 1;
/// indicates that the middle mouse button is down.
pub const EVENT_FLAG_MBUTTON: i32 = 4;
/// indicates that the right mouse button is down.
pub const EVENT_FLAG_RBUTTON: i32 = 2;
/// indicates that SHIFT Key is pressed.
pub const EVENT_FLAG_SHIFTKEY: i32 = 16;
/// indicates that left mouse button is double clicked.
pub const EVENT_LBUTTONDBLCLK: i32 = 7;
/// indicates that the left mouse button is pressed.
pub const EVENT_LBUTTONDOWN: i32 = 1;
/// indicates that left mouse button is released.
pub const EVENT_LBUTTONUP: i32 = 4;
/// indicates that middle mouse button is double clicked.
pub const EVENT_MBUTTONDBLCLK: i32 = 9;
/// indicates that the middle mouse button is pressed.
pub const EVENT_MBUTTONDOWN: i32 = 3;
/// indicates that middle mouse button is released.
pub const EVENT_MBUTTONUP: i32 = 6;
/// positive and negative values mean right and left scrolling, respectively.
pub const EVENT_MOUSEHWHEEL: i32 = 11;
/// indicates that the mouse pointer has moved over the window.
pub const EVENT_MOUSEMOVE: i32 = 0;
/// positive and negative values mean forward and backward scrolling, respectively.
pub const EVENT_MOUSEWHEEL: i32 = 10;
/// indicates that right mouse button is double clicked.
pub const EVENT_RBUTTONDBLCLK: i32 = 8;
/// indicates that the right mouse button is pressed.
pub const EVENT_RBUTTONDOWN: i32 = 2;
/// indicates that right mouse button is released.
pub const EVENT_RBUTTONUP: i32 = 5;
/// Checkbox button.
pub const QT_CHECKBOX: i32 = 1;
/// Weight of 87
pub const QT_FONT_BLACK: i32 = 87;
/// Weight of 75
pub const QT_FONT_BOLD: i32 = 75;
/// Weight of 63
pub const QT_FONT_DEMIBOLD: i32 = 63;
/// Weight of 25
pub const QT_FONT_LIGHT: i32 = 25;
/// Weight of 50
pub const QT_FONT_NORMAL: i32 = 50;
/// Button should create a new buttonbar
pub const QT_NEW_BUTTONBAR: i32 = 1024;
/// Push button.
pub const QT_PUSH_BUTTON: i32 = 0;
/// Radiobox button.
pub const QT_RADIOBOX: i32 = 2;
/// Italic font.
pub const QT_STYLE_ITALIC: i32 = 1;
/// Normal font.
pub const QT_STYLE_NORMAL: i32 = 0;
/// Oblique font.
pub const QT_STYLE_OBLIQUE: i32 = 2;
/// the user cannot resize the window, the size is constrainted by the image displayed.
pub const WINDOW_AUTOSIZE: i32 = 1;
/// the image expends as much as it can (no ratio constraint).
pub const WINDOW_FREERATIO: i32 = 256;
/// change the window to fullscreen.
pub const WINDOW_FULLSCREEN: i32 = 1;
/// status bar and tool bar
pub const WINDOW_GUI_EXPANDED: i32 = 0;
/// old fashious way
pub const WINDOW_GUI_NORMAL: i32 = 16;
/// the ratio of the image is respected.
pub const WINDOW_KEEPRATIO: i32 = 0;
/// the user can resize the window (no constraint) / also use to switch a fullscreen window to a normal size.
pub const WINDOW_NORMAL: i32 = 0;
/// window with opengl support.
pub const WINDOW_OPENGL: i32 = 4096;
/// window's aspect ration (can be set to WINDOW_FREERATIO or WINDOW_KEEPRATIO).
pub const WND_PROP_ASPECT_RATIO: i32 = 2;
/// autosize property      (can be WINDOW_NORMAL or WINDOW_AUTOSIZE).
pub const WND_PROP_AUTOSIZE: i32 = 1;
/// fullscreen property    (can be WINDOW_NORMAL or WINDOW_FULLSCREEN).
pub const WND_PROP_FULLSCREEN: i32 = 0;
/// opengl support.
pub const WND_PROP_OPENGL: i32 = 3;
/// checks whether the window exists and is visible
pub const WND_PROP_VISIBLE: i32 = 4;
/// Mouse Event Flags see cv::MouseCallback
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseEventFlags {
	/// indicates that the left mouse button is down.
	EVENT_FLAG_LBUTTON = 1 as isize,
	/// indicates that the right mouse button is down.
	EVENT_FLAG_RBUTTON = 2 as isize,
	/// indicates that the middle mouse button is down.
	EVENT_FLAG_MBUTTON = 4 as isize,
	/// indicates that CTRL Key is pressed.
	EVENT_FLAG_CTRLKEY = 8 as isize,
	/// indicates that SHIFT Key is pressed.
	EVENT_FLAG_SHIFTKEY = 16 as isize,
	/// indicates that ALT Key is pressed.
	EVENT_FLAG_ALTKEY = 32 as isize,
}

/// Mouse Events see cv::MouseCallback
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseEventTypes {
	/// indicates that the mouse pointer has moved over the window.
	EVENT_MOUSEMOVE = 0 as isize,
	/// indicates that the left mouse button is pressed.
	EVENT_LBUTTONDOWN = 1 as isize,
	/// indicates that the right mouse button is pressed.
	EVENT_RBUTTONDOWN = 2 as isize,
	/// indicates that the middle mouse button is pressed.
	EVENT_MBUTTONDOWN = 3 as isize,
	/// indicates that left mouse button is released.
	EVENT_LBUTTONUP = 4 as isize,
	/// indicates that right mouse button is released.
	EVENT_RBUTTONUP = 5 as isize,
	/// indicates that middle mouse button is released.
	EVENT_MBUTTONUP = 6 as isize,
	/// indicates that left mouse button is double clicked.
	EVENT_LBUTTONDBLCLK = 7 as isize,
	/// indicates that right mouse button is double clicked.
	EVENT_RBUTTONDBLCLK = 8 as isize,
	/// indicates that middle mouse button is double clicked.
	EVENT_MBUTTONDBLCLK = 9 as isize,
	/// positive and negative values mean forward and backward scrolling, respectively.
	EVENT_MOUSEWHEEL = 10 as isize,
	/// positive and negative values mean right and left scrolling, respectively.
	EVENT_MOUSEHWHEEL = 11 as isize,
}

/// Qt "button" type
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum QtButtonTypes {
	/// Push button.
	QT_PUSH_BUTTON = 0 as isize,
	/// Checkbox button.
	QT_CHECKBOX = 1 as isize,
	/// Radiobox button.
	QT_RADIOBOX = 2 as isize,
	/// Button should create a new buttonbar
	QT_NEW_BUTTONBAR = 1024 as isize,
}

/// Qt font style
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum QtFontStyles {
	/// Normal font.
	QT_STYLE_NORMAL = 0 as isize,
	/// Italic font.
	QT_STYLE_ITALIC = 1 as isize,
	/// Oblique font.
	QT_STYLE_OBLIQUE = 2 as isize,
}

/// Qt font weight
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum QtFontWeights {
	/// Weight of 25
	QT_FONT_LIGHT = 25 as isize,
	/// Weight of 50
	QT_FONT_NORMAL = 50 as isize,
	/// Weight of 63
	QT_FONT_DEMIBOLD = 63 as isize,
	/// Weight of 75
	QT_FONT_BOLD = 75 as isize,
	/// Weight of 87
	QT_FONT_BLACK = 87 as isize,
}

/// Flags for cv::namedWindow
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WindowFlags {
	/// the user can resize the window (no constraint) / also use to switch a fullscreen window to a normal size.
	WINDOW_NORMAL = 0 as isize,
	/// the user cannot resize the window, the size is constrainted by the image displayed.
	WINDOW_AUTOSIZE = 1 as isize,
	/// window with opengl support.
	WINDOW_OPENGL = 4096 as isize,
	// change the window to fullscreen.
	// WINDOW_FULLSCREEN = 1 as isize, // duplicate discriminant
	/// the image expends as much as it can (no ratio constraint).
	WINDOW_FREERATIO = 256 as isize,
	// the ratio of the image is respected.
	// WINDOW_KEEPRATIO = 0 as isize, // duplicate discriminant
	// status bar and tool bar
	// WINDOW_GUI_EXPANDED = 0 as isize, // duplicate discriminant
	/// old fashious way
	WINDOW_GUI_NORMAL = 16 as isize,
}

/// Flags for cv::setWindowProperty / cv::getWindowProperty
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WindowPropertyFlags {
	/// fullscreen property    (can be WINDOW_NORMAL or WINDOW_FULLSCREEN).
	WND_PROP_FULLSCREEN = 0 as isize,
	/// autosize property      (can be WINDOW_NORMAL or WINDOW_AUTOSIZE).
	WND_PROP_AUTOSIZE = 1 as isize,
	/// window's aspect ration (can be set to WINDOW_FREERATIO or WINDOW_KEEPRATIO).
	WND_PROP_ASPECT_RATIO = 2 as isize,
	/// opengl support.
	WND_PROP_OPENGL = 3 as isize,
	/// checks whether the window exists and is visible
	WND_PROP_VISIBLE = 4 as isize,
}

/// Callback function for a button created by cv::createButton
/// ## Parameters
/// * state: current state of the button. It could be -1 for a push button, 0 or 1 for a check/radio box button.
/// * userdata: The optional parameter.
pub type ButtonCallback = Option<Box<dyn FnMut(i32) -> () + Send + Sync + 'static>>;
/// Callback function for mouse events. see cv::setMouseCallback
/// ## Parameters
/// * event: one of the cv::MouseEventTypes constants.
/// * x: The x-coordinate of the mouse event.
/// * y: The y-coordinate of the mouse event.
/// * flags: one of the cv::MouseEventFlags constants.
/// * userdata: The optional parameter.
pub type MouseCallback = Option<Box<dyn FnMut(i32, i32, i32, i32) -> () + Send + Sync + 'static>>;
/// Callback function defined to be called every frame. See cv::setOpenGlDrawCallback
/// ## Parameters
/// * userdata: The optional parameter.
pub type OpenGlDrawCallback = Option<Box<dyn FnMut() -> () + Send + Sync + 'static>>;
/// Callback function for Trackbar see cv::createTrackbar
/// ## Parameters
/// * pos: current position of the specified trackbar.
/// * userdata: The optional parameter.
pub type TrackbarCallback = Option<Box<dyn FnMut(i32) -> () + Send + Sync + 'static>>;
/// Draws a text on the image.
/// 
/// The function addText draws *text* on the image *img* using a specific font *font* (see example cv::fontQt
/// )
/// 
/// ## Parameters
/// * img: 8-bit 3-channel image where the text should be drawn.
/// * text: Text to write on an image.
/// * org: Point(x,y) where the text should start on an image.
/// * font: Font to use to draw a text.
pub fn add_text(img: &core::Mat, text: &str, org: core::Point, font: &crate::highgui::QtFont) -> Result<()> {
	string_arg!(text);
	unsafe { sys::cv_addText_const_MatX_const_StringX_Point_const_QtFontX(img.as_raw_Mat(), text.as_ptr(), &org, font.as_raw_QtFont()) }.into_result()
}

/// Draws a text on the image.
/// 
/// ## Parameters
/// * img: 8-bit 3-channel image where the text should be drawn.
/// * text: Text to write on an image.
/// * org: Point(x,y) where the text should start on an image.
/// * nameFont: Name of the font. The name should match the name of a system font (such as
/// *Times*). If the font is not found, a default one is used.
/// * pointSize: Size of the font. If not specified, equal zero or negative, the point size of the
/// font is set to a system-dependent default value. Generally, this is 12 points.
/// * color: Color of the font in BGRA where A = 255 is fully transparent.
/// * weight: Font weight. Available operation flags are : cv::QtFontWeights You can also specify a positive integer for better control.
/// * style: Font style. Available operation flags are : cv::QtFontStyles
/// * spacing: Spacing between characters. It can be negative or positive.
/// 
/// ## C++ default parameters
/// * point_size: -1
/// * color: Scalar::all(0)
/// * weight: QT_FONT_NORMAL
/// * style: QT_STYLE_NORMAL
/// * spacing: 0
pub fn add_text_with_font(img: &core::Mat, text: &str, org: core::Point, name_font: &str, point_size: i32, color: core::Scalar, weight: i32, style: i32, spacing: i32) -> Result<()> {
	string_arg!(text);
	string_arg!(name_font);
	unsafe { sys::cv_addText_const_MatX_const_StringX_Point_const_StringX_int_Scalar_int_int_int(img.as_raw_Mat(), text.as_ptr(), &org, name_font.as_ptr(), point_size, &color, weight, style, spacing) }.into_result()
}

/// Attaches a button to the control panel.
/// 
/// The function createButton attaches a button to the control panel. Each button is added to a
/// buttonbar to the right of the last button. A new buttonbar is created if nothing was attached to the
/// control panel before, or if the last element attached to the control panel was a trackbar or if the
/// QT_NEW_BUTTONBAR flag is added to the type.
/// 
/// See below various examples of the cv::createButton function call: :
/// ```ignore
///    createButton(NULL,callbackButton);//create a push button "button 0", that will call callbackButton.
///    createButton("button2",callbackButton,NULL,QT_CHECKBOX,0);
///    createButton("button3",callbackButton,&value);
///    createButton("button5",callbackButton1,NULL,QT_RADIOBOX);
///    createButton("button6",callbackButton2,NULL,QT_PUSH_BUTTON,1);
///    createButton("button6",callbackButton2,NULL,QT_PUSH_BUTTON|QT_NEW_BUTTONBAR);// create a push button in a new row
/// ```
/// 
/// 
/// ## Parameters
/// * bar_name: Name of the button.
/// * on_change: Pointer to the function to be called every time the button changes its state.
/// This function should be prototyped as void Foo(int state,\*void); . *state* is the current state
/// of the button. It could be -1 for a push button, 0 or 1 for a check/radio box button.
/// * userdata: Pointer passed to the callback function.
/// * type: Optional type of the button. Available types are: (cv::QtButtonTypes)
/// * initial_button_state: Default state of the button. Use for checkbox and radiobox. Its
/// value could be 0 or 1. (__Optional__)
/// 
/// ## C++ default parameters
/// * userdata: 0
/// * typ: QT_PUSH_BUTTON
/// * initial_button_state: false
pub fn create_button(bar_name: &str, on_change: crate::highgui::ButtonCallback, typ: i32, initial_button_state: bool) -> Result<i32> {
	string_arg!(bar_name);
	callback_arg!(on_change_trampoline(state: i32, userdata: *mut c_void) -> () => userdata in callbacks => on_change(state: i32) -> ());
	userdata_arg!(userdata in callbacks => on_change);
	unsafe { sys::cv_createButton_const_StringX_ButtonCallback_voidX_int_bool(bar_name.as_ptr(), on_change_trampoline, userdata, typ, initial_button_state) }.into_result()
}

/// Creates a trackbar and attaches it to the specified window.
/// 
/// The function createTrackbar creates a trackbar (a slider or range control) with the specified name
/// and range, assigns a variable value to be a position synchronized with the trackbar and specifies
/// the callback function onChange to be called on the trackbar position change. The created trackbar is
/// displayed in the specified window winname.
/// 
/// 
/// Note:
/// 
/// [__Qt Backend Only__] winname can be empty (or NULL) if the trackbar should be attached to the
/// control panel.
/// 
/// Clicking the label of each trackbar enables editing the trackbar values manually.
/// 
/// ## Parameters
/// * trackbarname: Name of the created trackbar.
/// * winname: Name of the window that will be used as a parent of the created trackbar.
/// * value: Optional pointer to an integer variable whose value reflects the position of the
/// slider. Upon creation, the slider position is defined by this variable.
/// * count: Maximal position of the slider. The minimal position is always 0.
/// * onChange: Pointer to the function to be called every time the slider changes position. This
/// function should be prototyped as void Foo(int,void\*); , where the first parameter is the trackbar
/// position and the second parameter is the user data (see the next parameter). If the callback is
/// the NULL pointer, no callbacks are called, but only value is updated.
/// * userdata: User data that is passed as is to the callback. It can be used to handle trackbar
/// events without using global variables.
/// 
/// ## C++ default parameters
/// * on_change: 0
/// * userdata: 0
pub fn create_trackbar(trackbarname: &str, winname: &str, value: &mut i32, count: i32, on_change: crate::highgui::TrackbarCallback) -> Result<i32> {
	string_arg!(trackbarname);
	string_arg!(winname);
	callback_arg!(on_change_trampoline(pos: i32, userdata: *mut c_void) -> () => userdata in callbacks => on_change(pos: i32) -> ());
	userdata_arg!(userdata in callbacks => on_change);
	unsafe { sys::cv_createTrackbar_const_StringX_const_StringX_intX_int_TrackbarCallback_voidX(trackbarname.as_ptr(), winname.as_ptr(), value, count, on_change_trampoline, userdata) }.into_result()
}

/// Destroys all of the HighGUI windows.
/// 
/// The function destroyAllWindows destroys all of the opened HighGUI windows.
pub fn destroy_all_windows() -> Result<()> {
	unsafe { sys::cv_destroyAllWindows() }.into_result()
}

/// Destroys the specified window.
/// 
/// The function destroyWindow destroys the window with the given name.
/// 
/// ## Parameters
/// * winname: Name of the window to be destroyed.
pub fn destroy_window(winname: &str) -> Result<()> {
	string_arg!(winname);
	unsafe { sys::cv_destroyWindow_const_StringX(winname.as_ptr()) }.into_result()
}

/// Displays a text on a window image as an overlay for a specified duration.
/// 
/// The function displayOverlay displays useful information/tips on top of the window for a certain
/// amount of time *delayms*. The function does not modify the image, displayed in the window, that is,
/// after the specified delay the original content of the window is restored.
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * text: Overlay text to write on a window image.
/// * delayms: The period (in milliseconds), during which the overlay text is displayed. If this
/// function is called before the previous overlay text timed out, the timer is restarted and the text
/// is updated. If this value is zero, the text never disappears.
/// 
/// ## C++ default parameters
/// * delayms: 0
pub fn display_overlay(winname: &str, text: &str, delayms: i32) -> Result<()> {
	string_arg!(winname);
	string_arg!(text);
	unsafe { sys::cv_displayOverlay_const_StringX_const_StringX_int(winname.as_ptr(), text.as_ptr(), delayms) }.into_result()
}

/// Displays a text on the window statusbar during the specified period of time.
/// 
/// The function displayStatusBar displays useful information/tips on top of the window for a certain
/// amount of time *delayms* . This information is displayed on the window statusbar (the window must be
/// created with the CV_GUI_EXPANDED flags).
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * text: Text to write on the window statusbar.
/// * delayms: Duration (in milliseconds) to display the text. If this function is called before
/// the previous text timed out, the timer is restarted and the text is updated. If this value is
/// zero, the text never disappears.
/// 
/// ## C++ default parameters
/// * delayms: 0
pub fn display_status_bar(winname: &str, text: &str, delayms: i32) -> Result<()> {
	string_arg!(winname);
	string_arg!(text);
	unsafe { sys::cv_displayStatusBar_const_StringX_const_StringX_int(winname.as_ptr(), text.as_ptr(), delayms) }.into_result()
}

/// Creates the font to draw a text on an image.
/// 
/// The function fontQt creates a cv::QtFont object. This cv::QtFont is not compatible with putText .
/// 
/// A basic usage of this function is the following: :
/// ```ignore
///    QtFont font = fontQt("Times");
///    addText( img1, "Hello World !", Point(50,50), font);
/// ```
/// 
/// 
/// ## Parameters
/// * nameFont: Name of the font. The name should match the name of a system font (such as
/// *Times*). If the font is not found, a default one is used.
/// * pointSize: Size of the font. If not specified, equal zero or negative, the point size of the
/// font is set to a system-dependent default value. Generally, this is 12 points.
/// * color: Color of the font in BGRA where A = 255 is fully transparent. Use the macro CV_RGB
/// for simplicity.
/// * weight: Font weight. Available operation flags are : cv::QtFontWeights You can also specify a positive integer for better control.
/// * style: Font style. Available operation flags are : cv::QtFontStyles
/// * spacing: Spacing between characters. It can be negative or positive.
/// 
/// ## C++ default parameters
/// * point_size: -1
/// * color: Scalar::all(0)
/// * weight: QT_FONT_NORMAL
/// * style: QT_STYLE_NORMAL
/// * spacing: 0
pub fn font_qt(name_font: &str, point_size: i32, color: core::Scalar, weight: i32, style: i32, spacing: i32) -> Result<crate::highgui::QtFont> {
	string_arg!(name_font);
	unsafe { sys::cv_fontQt_const_StringX_int_Scalar_int_int_int(name_font.as_ptr(), point_size, &color, weight, style, spacing) }.into_result().map(|ptr| unsafe { crate::highgui::QtFont::from_raw(ptr) })
}

/// Gets the mouse-wheel motion delta, when handling mouse-wheel events cv::EVENT_MOUSEWHEEL and
/// cv::EVENT_MOUSEHWHEEL.
/// 
/// For regular mice with a scroll-wheel, delta will be a multiple of 120. The value 120 corresponds to
/// a one notch rotation of the wheel or the threshold for action to be taken and one such action should
/// occur for each delta. Some high-precision mice with higher-resolution freely-rotating wheels may
/// generate smaller values.
/// 
/// For cv::EVENT_MOUSEWHEEL positive and negative values mean forward and backward scrolling,
/// respectively. For cv::EVENT_MOUSEHWHEEL, where available, positive and negative values mean right and
/// left scrolling, respectively.
/// 
/// With the C API, the macro CV_GET_WHEEL_DELTA(flags) can be used alternatively.
/// 
/// 
/// Note:
/// 
/// Mouse-wheel events are currently supported only on Windows.
/// 
/// ## Parameters
/// * flags: The mouse callback flags parameter.
pub fn get_mouse_wheel_delta(flags: i32) -> Result<i32> {
	unsafe { sys::cv_getMouseWheelDelta_int(flags) }.into_result()
}

/// Returns the trackbar position.
/// 
/// The function returns the current position of the specified trackbar.
/// 
/// 
/// Note:
/// 
/// [__Qt Backend Only__] winname can be empty (or NULL) if the trackbar is attached to the control
/// panel.
/// 
/// ## Parameters
/// * trackbarname: Name of the trackbar.
/// * winname: Name of the window that is the parent of the trackbar.
pub fn get_trackbar_pos(trackbarname: &str, winname: &str) -> Result<i32> {
	string_arg!(trackbarname);
	string_arg!(winname);
	unsafe { sys::cv_getTrackbarPos_const_StringX_const_StringX(trackbarname.as_ptr(), winname.as_ptr()) }.into_result()
}

/// Provides parameters of a window.
/// 
/// The function getWindowProperty returns properties of a window.
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * prop_id: Window property to retrieve. The following operation flags are available: (cv::WindowPropertyFlags)
/// ## See also
/// setWindowProperty
pub fn get_window_property(winname: &str, prop_id: i32) -> Result<f64> {
	string_arg!(winname);
	unsafe { sys::cv_getWindowProperty_const_StringX_int(winname.as_ptr(), prop_id) }.into_result()
}

/// Displays an image in the specified window.
/// 
/// The function imshow displays an image in the specified window. If the window was created with the
/// cv::WINDOW_AUTOSIZE flag, the image is shown with its original size, however it is still limited by the screen resolution.
/// Otherwise, the image is scaled to fit the window. The function may scale the image, depending on its depth:
/// 
/// *   If the image is 8-bit unsigned, it is displayed as is.
/// *   If the image is 16-bit unsigned or 32-bit integer, the pixels are divided by 256. That is, the
///    value range [0,255\*256] is mapped to [0,255].
/// *   If the image is 32-bit floating-point, the pixel values are multiplied by 255. That is, the
///    value range [0,1] is mapped to [0,255].
/// 
/// If window was created with OpenGL support, cv::imshow also support ogl::Buffer , ogl::Texture2D and
/// cuda::GpuMat as input.
/// 
/// If the window was not created before this function, it is assumed creating a window with cv::WINDOW_AUTOSIZE.
/// 
/// If you need to show an image that is bigger than the screen resolution, you will need to call namedWindow("", WINDOW_NORMAL) before the imshow.
/// 
/// 
/// Note: This function should be followed by cv::waitKey function which displays the image for specified
/// milliseconds. Otherwise, it won't display the image. For example, **waitKey(0)** will display the window
/// infinitely until any keypress (it is suitable for image display). **waitKey(25)** will display a frame
/// for 25 ms, after which display will be automatically closed. (If you put it in a loop to read
/// videos, it will display the video frame-by-frame)
/// 
/// 
/// Note:
/// 
/// [__Windows Backend Only__] Pressing Ctrl+C will copy the image to the clipboard.
/// 
/// [__Windows Backend Only__] Pressing Ctrl+S will show a dialog to save the image.
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * mat: Image to be shown.
pub fn imshow(winname: &str, mat: &dyn core::ToInputArray) -> Result<()> {
	string_arg!(winname);
	input_array_arg!(mat);
	unsafe { sys::cv_imshow_const_StringX_const__InputArrayX(winname.as_ptr(), mat.as_raw__InputArray()) }.into_result()
}

/// Loads parameters of the specified window.
/// 
/// The function loadWindowParameters loads size, location, flags, trackbars value, zoom and panning
/// location of the window windowName.
/// 
/// ## Parameters
/// * windowName: Name of the window.
pub fn load_window_parameters(window_name: &str) -> Result<()> {
	string_arg!(window_name);
	unsafe { sys::cv_loadWindowParameters_const_StringX(window_name.as_ptr()) }.into_result()
}

/// Moves window to the specified position
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * x: The new x-coordinate of the window.
/// * y: The new y-coordinate of the window.
pub fn move_window(winname: &str, x: i32, y: i32) -> Result<()> {
	string_arg!(winname);
	unsafe { sys::cv_moveWindow_const_StringX_int_int(winname.as_ptr(), x, y) }.into_result()
}

/// Creates a window.
/// 
/// The function namedWindow creates a window that can be used as a placeholder for images and
/// trackbars. Created windows are referred to by their names.
/// 
/// If a window with the same name already exists, the function does nothing.
/// 
/// You can call cv::destroyWindow or cv::destroyAllWindows to close the window and de-allocate any associated
/// memory usage. For a simple program, you do not really have to call these functions because all the
/// resources and windows of the application are closed automatically by the operating system upon exit.
/// 
/// 
/// Note:
/// 
/// Qt backend supports additional flags:
///  *   **WINDOW_NORMAL or WINDOW_AUTOSIZE:** WINDOW_NORMAL enables you to resize the
///      window, whereas WINDOW_AUTOSIZE adjusts automatically the window size to fit the
///      displayed image (see imshow ), and you cannot change the window size manually.
///  *   **WINDOW_FREERATIO or WINDOW_KEEPRATIO:** WINDOW_FREERATIO adjusts the image
///      with no respect to its ratio, whereas WINDOW_KEEPRATIO keeps the image ratio.
///  *   **WINDOW_GUI_NORMAL or WINDOW_GUI_EXPANDED:** WINDOW_GUI_NORMAL is the old way to draw the window
///      without statusbar and toolbar, whereas WINDOW_GUI_EXPANDED is a new enhanced GUI.
/// By default, flags == WINDOW_AUTOSIZE | WINDOW_KEEPRATIO | WINDOW_GUI_EXPANDED
/// 
/// ## Parameters
/// * winname: Name of the window in the window caption that may be used as a window identifier.
/// * flags: Flags of the window. The supported flags are: (cv::WindowFlags)
/// 
/// ## C++ default parameters
/// * flags: WINDOW_AUTOSIZE
pub fn named_window(winname: &str, flags: i32) -> Result<()> {
	string_arg!(winname);
	unsafe { sys::cv_namedWindow_const_StringX_int(winname.as_ptr(), flags) }.into_result()
}

/// Resizes window to the specified size
/// 
/// 
/// Note:
/// 
/// *   The specified window size is for the image area. Toolbars are not counted.
/// *   Only windows created without cv::WINDOW_AUTOSIZE flag can be resized.
/// 
/// ## Parameters
/// * winname: Window name.
/// * width: The new window width.
/// * height: The new window height.
pub fn resize_window(winname: &str, width: i32, height: i32) -> Result<()> {
	string_arg!(winname);
	unsafe { sys::cv_resizeWindow_const_StringX_int_int(winname.as_ptr(), width, height) }.into_result()
}

/// Saves parameters of the specified window.
/// 
/// The function saveWindowParameters saves size, location, flags, trackbars value, zoom and panning
/// location of the window windowName.
/// 
/// ## Parameters
/// * windowName: Name of the window.
pub fn save_window_parameters(window_name: &str) -> Result<()> {
	string_arg!(window_name);
	unsafe { sys::cv_saveWindowParameters_const_StringX(window_name.as_ptr()) }.into_result()
}

/// Sets mouse handler for the specified window
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * onMouse: Mouse callback. See OpenCV samples, such as
/// <https://github.com/opencv/opencv/tree/master/samples/cpp/ffilldemo.cpp>, on how to specify and
/// use the callback.
/// * userdata: The optional parameter passed to the callback.
/// 
/// ## C++ default parameters
/// * userdata: 0
pub fn set_mouse_callback(winname: &str, on_mouse: crate::highgui::MouseCallback) -> Result<()> {
	string_arg!(winname);
	callback_arg!(on_mouse_trampoline(event: i32, x: i32, y: i32, flags: i32, userdata: *mut c_void) -> () => userdata in callbacks => on_mouse(event: i32, x: i32, y: i32, flags: i32) -> ());
	userdata_arg!(userdata in callbacks => on_mouse);
	unsafe { sys::cv_setMouseCallback_const_StringX_MouseCallback_voidX(winname.as_ptr(), on_mouse_trampoline, userdata) }.into_result()
}

/// Sets the specified window as current OpenGL context.
/// 
/// ## Parameters
/// * winname: Name of the window.
pub fn set_opengl_context(winname: &str) -> Result<()> {
	string_arg!(winname);
	unsafe { sys::cv_setOpenGlContext_const_StringX(winname.as_ptr()) }.into_result()
}

/// Sets a callback function to be called to draw on top of displayed image.
/// 
/// The function setOpenGlDrawCallback can be used to draw 3D data on the window. See the example of
/// callback function below:
/// ```ignore
///    void on_opengl(void* param)
///    {
///        glLoadIdentity();
/// 
///        glTranslated(0.0, 0.0, -1.0);
/// 
///        glRotatef( 55, 1, 0, 0 );
///        glRotatef( 45, 0, 1, 0 );
///        glRotatef( 0, 0, 0, 1 );
/// 
///        static const int coords[6][4][3] = {
///            { { +1, -1, -1 }, { -1, -1, -1 }, { -1, +1, -1 }, { +1, +1, -1 } },
///            { { +1, +1, -1 }, { -1, +1, -1 }, { -1, +1, +1 }, { +1, +1, +1 } },
///            { { +1, -1, +1 }, { +1, -1, -1 }, { +1, +1, -1 }, { +1, +1, +1 } },
///            { { -1, -1, -1 }, { -1, -1, +1 }, { -1, +1, +1 }, { -1, +1, -1 } },
///            { { +1, -1, +1 }, { -1, -1, +1 }, { -1, -1, -1 }, { +1, -1, -1 } },
///            { { -1, -1, +1 }, { +1, -1, +1 }, { +1, +1, +1 }, { -1, +1, +1 } }
///        };
/// 
///        for (int i = 0; i < 6; ++i) {
///                    glColor3ub( i*20, 100+i*10, i*42 );
///                    glBegin(GL_QUADS);
///                    for (int j = 0; j < 4; ++j) {
///                             glVertex3d(0.2 * coords[i][j][0], 0.2 * coords[i][j][1], 0.2 * coords[i][j][2]);
///                    }
///                    glEnd();
///        }
///    }
/// ```
/// 
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * onOpenGlDraw: Pointer to the function to be called every frame. This function should be
/// prototyped as void Foo(void\*) .
/// * userdata: Pointer passed to the callback function.(__Optional__)
/// 
/// ## C++ default parameters
/// * userdata: 0
pub fn set_opengl_draw_callback(winname: &str, on_opengl_draw: crate::highgui::OpenGlDrawCallback) -> Result<()> {
	string_arg!(winname);
	callback_arg!(on_opengl_draw_trampoline(userdata: *mut c_void) -> () => userdata in callbacks => on_opengl_draw() -> ());
	userdata_arg!(userdata in callbacks => on_opengl_draw);
	unsafe { sys::cv_setOpenGlDrawCallback_const_StringX_OpenGlDrawCallback_voidX(winname.as_ptr(), on_opengl_draw_trampoline, userdata) }.into_result()
}

/// Sets the trackbar maximum position.
/// 
/// The function sets the maximum position of the specified trackbar in the specified window.
/// 
/// 
/// Note:
/// 
/// [__Qt Backend Only__] winname can be empty (or NULL) if the trackbar is attached to the control
/// panel.
/// 
/// ## Parameters
/// * trackbarname: Name of the trackbar.
/// * winname: Name of the window that is the parent of trackbar.
/// * maxval: New maximum position.
pub fn set_trackbar_max(trackbarname: &str, winname: &str, maxval: i32) -> Result<()> {
	string_arg!(trackbarname);
	string_arg!(winname);
	unsafe { sys::cv_setTrackbarMax_const_StringX_const_StringX_int(trackbarname.as_ptr(), winname.as_ptr(), maxval) }.into_result()
}

/// Sets the trackbar minimum position.
/// 
/// The function sets the minimum position of the specified trackbar in the specified window.
/// 
/// 
/// Note:
/// 
/// [__Qt Backend Only__] winname can be empty (or NULL) if the trackbar is attached to the control
/// panel.
/// 
/// ## Parameters
/// * trackbarname: Name of the trackbar.
/// * winname: Name of the window that is the parent of trackbar.
/// * minval: New maximum position.
pub fn set_trackbar_min(trackbarname: &str, winname: &str, minval: i32) -> Result<()> {
	string_arg!(trackbarname);
	string_arg!(winname);
	unsafe { sys::cv_setTrackbarMin_const_StringX_const_StringX_int(trackbarname.as_ptr(), winname.as_ptr(), minval) }.into_result()
}

/// Sets the trackbar position.
/// 
/// The function sets the position of the specified trackbar in the specified window.
/// 
/// 
/// Note:
/// 
/// [__Qt Backend Only__] winname can be empty (or NULL) if the trackbar is attached to the control
/// panel.
/// 
/// ## Parameters
/// * trackbarname: Name of the trackbar.
/// * winname: Name of the window that is the parent of trackbar.
/// * pos: New position.
pub fn set_trackbar_pos(trackbarname: &str, winname: &str, pos: i32) -> Result<()> {
	string_arg!(trackbarname);
	string_arg!(winname);
	unsafe { sys::cv_setTrackbarPos_const_StringX_const_StringX_int(trackbarname.as_ptr(), winname.as_ptr(), pos) }.into_result()
}

/// Changes parameters of a window dynamically.
/// 
/// The function setWindowProperty enables changing properties of a window.
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * prop_id: Window property to edit. The supported operation flags are: (cv::WindowPropertyFlags)
/// * prop_value: New value of the window property. The supported flags are: (cv::WindowFlags)
pub fn set_window_property(winname: &str, prop_id: i32, prop_value: f64) -> Result<()> {
	string_arg!(winname);
	unsafe { sys::cv_setWindowProperty_const_StringX_int_double(winname.as_ptr(), prop_id, prop_value) }.into_result()
}

/// Updates window title
/// ## Parameters
/// * winname: Name of the window.
/// * title: New title.
pub fn set_window_title(winname: &str, title: &str) -> Result<()> {
	string_arg!(winname);
	string_arg!(title);
	unsafe { sys::cv_setWindowTitle_const_StringX_const_StringX(winname.as_ptr(), title.as_ptr()) }.into_result()
}

pub fn start_loop(pt2_func: Option<unsafe extern "C" fn(i32, *mut *mut c_char) -> i32>, argc: i32, argv: &mut [&str]) -> Result<i32> {
	string_array_arg_mut!(argv);
	unsafe { sys::cv_startLoop_int__X__int__charXX__int_charXX(pt2_func, argc, argv.as_mut_ptr()) }.into_result()
}

pub fn start_window_thread() -> Result<i32> {
	unsafe { sys::cv_startWindowThread() }.into_result()
}

pub fn stop_loop() -> Result<()> {
	unsafe { sys::cv_stopLoop() }.into_result()
}

/// Force window to redraw its context and call draw callback ( See cv::setOpenGlDrawCallback ).
/// 
/// ## Parameters
/// * winname: Name of the window.
pub fn update_window(winname: &str) -> Result<()> {
	string_arg!(winname);
	unsafe { sys::cv_updateWindow_const_StringX(winname.as_ptr()) }.into_result()
}

/// Similar to #waitKey, but returns full key code.
/// 
/// 
/// Note:
/// 
/// Key code is implementation specific and depends on used backend: QT/GTK/Win32/etc
/// 
/// ## C++ default parameters
/// * delay: 0
pub fn wait_key_ex(delay: i32) -> Result<i32> {
	unsafe { sys::cv_waitKeyEx_int(delay) }.into_result()
}

/// Waits for a pressed key.
/// 
/// The function waitKey waits for a key event infinitely (when ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdelay%7D%5Cleq%200) ) or for delay
/// milliseconds, when it is positive. Since the OS has a minimum time between switching threads, the
/// function will not wait exactly delay ms, it will wait at least delay ms, depending on what else is
/// running on your computer at that time. It returns the code of the pressed key or -1 if no key was
/// pressed before the specified time had elapsed.
/// 
/// 
/// Note:
/// 
/// This function is the only method in HighGUI that can fetch and handle events, so it needs to be
/// called periodically for normal event processing unless HighGUI is used within an environment that
/// takes care of event processing.
/// 
/// 
/// Note:
/// 
/// The function only works if there is at least one HighGUI window created and the window is active.
/// If there are several HighGUI windows, any of them can be active.
/// 
/// ## Parameters
/// * delay: Delay in milliseconds. 0 is the special value that means "forever".
/// 
/// ## C++ default parameters
/// * delay: 0
pub fn wait_key(delay: i32) -> Result<i32> {
	unsafe { sys::cv_waitKey_int(delay) }.into_result()
}

/// QtFont available only for Qt. See cv::fontQt
pub trait QtFontTrait {
	fn as_raw_QtFont(&self) -> *const c_void;
	fn as_raw_mut_QtFont(&mut self) -> *mut c_void;

	/// Name of the font
	fn name_font(&self) -> String {
		unsafe { sys::cv_QtFont_nameFont_const(self.as_raw_QtFont()) }.into_result().map(|s| unsafe { crate::templ::receive_string(s as *mut String) }).expect("Infallible function failed: name_font")
	}
	
	/// Color of the font. Scalar(blue_component, green_component, red_component[, alpha_component])
	fn color(&self) -> core::Scalar {
		unsafe { sys::cv_QtFont_color_const(self.as_raw_QtFont()) }.into_result().expect("Infallible function failed: color")
	}
	
	/// Color of the font. Scalar(blue_component, green_component, red_component[, alpha_component])
	fn set_color(&mut self, val: core::Scalar) -> () {
		unsafe { sys::cv_QtFont_setColor_Scalar(self.as_raw_mut_QtFont(), &val) }.into_result().expect("Infallible function failed: set_color")
	}
	
	/// See cv::QtFontStyles
	fn font_face(&self) -> i32 {
		unsafe { sys::cv_QtFont_font_face_const(self.as_raw_QtFont()) }.into_result().expect("Infallible function failed: font_face")
	}
	
	/// See cv::QtFontStyles
	fn set_font_face(&mut self, val: i32) -> () {
		unsafe { sys::cv_QtFont_setFont_face_int(self.as_raw_mut_QtFont(), val) }.into_result().expect("Infallible function failed: set_font_face")
	}
	
	/// font data and metrics
	fn ascii(&self) -> &i32 {
		unsafe { sys::cv_QtFont_ascii_const(self.as_raw_QtFont()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: ascii")
	}
	
	fn greek(&self) -> &i32 {
		unsafe { sys::cv_QtFont_greek_const(self.as_raw_QtFont()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: greek")
	}
	
	fn cyrillic(&self) -> &i32 {
		unsafe { sys::cv_QtFont_cyrillic_const(self.as_raw_QtFont()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: cyrillic")
	}
	
	fn hscale(&self) -> f32 {
		unsafe { sys::cv_QtFont_hscale_const(self.as_raw_QtFont()) }.into_result().expect("Infallible function failed: hscale")
	}
	
	fn set_hscale(&mut self, val: f32) -> () {
		unsafe { sys::cv_QtFont_setHscale_float(self.as_raw_mut_QtFont(), val) }.into_result().expect("Infallible function failed: set_hscale")
	}
	
	fn vscale(&self) -> f32 {
		unsafe { sys::cv_QtFont_vscale_const(self.as_raw_QtFont()) }.into_result().expect("Infallible function failed: vscale")
	}
	
	fn set_vscale(&mut self, val: f32) -> () {
		unsafe { sys::cv_QtFont_setVscale_float(self.as_raw_mut_QtFont(), val) }.into_result().expect("Infallible function failed: set_vscale")
	}
	
	/// slope coefficient: 0 - normal, >0 - italic
	fn shear(&self) -> f32 {
		unsafe { sys::cv_QtFont_shear_const(self.as_raw_QtFont()) }.into_result().expect("Infallible function failed: shear")
	}
	
	/// slope coefficient: 0 - normal, >0 - italic
	fn set_shear(&mut self, val: f32) -> () {
		unsafe { sys::cv_QtFont_setShear_float(self.as_raw_mut_QtFont(), val) }.into_result().expect("Infallible function failed: set_shear")
	}
	
	/// See cv::QtFontWeights
	fn thickness(&self) -> i32 {
		unsafe { sys::cv_QtFont_thickness_const(self.as_raw_QtFont()) }.into_result().expect("Infallible function failed: thickness")
	}
	
	/// See cv::QtFontWeights
	fn set_thickness(&mut self, val: i32) -> () {
		unsafe { sys::cv_QtFont_setThickness_int(self.as_raw_mut_QtFont(), val) }.into_result().expect("Infallible function failed: set_thickness")
	}
	
	/// horizontal interval between letters
	fn dx(&self) -> f32 {
		unsafe { sys::cv_QtFont_dx_const(self.as_raw_QtFont()) }.into_result().expect("Infallible function failed: dx")
	}
	
	/// horizontal interval between letters
	fn set_dx(&mut self, val: f32) -> () {
		unsafe { sys::cv_QtFont_setDx_float(self.as_raw_mut_QtFont(), val) }.into_result().expect("Infallible function failed: set_dx")
	}
	
	/// PointSize
	fn line_type(&self) -> i32 {
		unsafe { sys::cv_QtFont_line_type_const(self.as_raw_QtFont()) }.into_result().expect("Infallible function failed: line_type")
	}
	
	/// PointSize
	fn set_line_type(&mut self, val: i32) -> () {
		unsafe { sys::cv_QtFont_setLine_type_int(self.as_raw_mut_QtFont(), val) }.into_result().expect("Infallible function failed: set_line_type")
	}
	
}

/// QtFont available only for Qt. See cv::fontQt
pub struct QtFont {
	ptr: *mut c_void
}

boxed_ptr! { QtFont }

impl Drop for QtFont {
	fn drop(&mut self) {
		extern "C" { fn cv_QtFont_delete(instance: *mut c_void); }
		unsafe { cv_QtFont_delete(self.as_raw_mut_QtFont()) };
	}
}

impl QtFont {
	pub fn as_raw_QtFont(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_QtFont(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for QtFont {}

impl crate::highgui::QtFontTrait for QtFont {
	fn as_raw_QtFont(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_QtFont(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl QtFont {
}
