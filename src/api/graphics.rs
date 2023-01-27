/*
struct playdate_graphics
{
	const struct playdate_video* video;

	// Drawing Functions
	void (*clear)(LCDColor color);
	void (*setBackgroundColor)(LCDSolidColor color);
	void (*setStencil)(LCDBitmap* stencil); // deprecated in favor of setStencilImage, which adds a "tile" flag
	void (*setDrawMode)(LCDBitmapDrawMode mode);
	void (*setDrawOffset)(int dx, int dy);
	void (*setClipRect)(int x, int y, int width, int height);
	void (*clearClipRect)(void);
	void (*setLineCapStyle)(LCDLineCapStyle endCapStyle);
	void (*setFont)(LCDFont* font);
	void (*setTextTracking)(int tracking);
	void (*pushContext)(LCDBitmap* target);
	void (*popContext)(void);

	void (*drawBitmap)(LCDBitmap* bitmap, int x, int y, LCDBitmapFlip flip);
	void (*tileBitmap)(LCDBitmap* bitmap, int x, int y, int width, int height, LCDBitmapFlip flip);
	void (*drawLine)(int x1, int y1, int x2, int y2, int width, LCDColor color);
	void (*fillTriangle)(int x1, int y1, int x2, int y2, int x3, int y3, LCDColor color);
	void (*drawRect)(int x, int y, int width, int height, LCDColor color);
	void (*fillRect)(int x, int y, int width, int height, LCDColor color);
	void (*drawEllipse)(int x, int y, int width, int height, int lineWidth, float startAngle, float endAngle, LCDColor color); // stroked inside the rect
	void (*fillEllipse)(int x, int y, int width, int height, float startAngle, float endAngle, LCDColor color);
	void (*drawScaledBitmap)(LCDBitmap* bitmap, int x, int y, float xscale, float yscale);
	int  (*drawText)(const void* text, size_t len, PDStringEncoding encoding, int x, int y);

	// LCDBitmap
	LCDBitmap* (*newBitmap)(int width, int height, LCDColor bgcolor);
	void (*freeBitmap)(LCDBitmap*);
	LCDBitmap* (*loadBitmap)(const char* path, const char** outerr);
	LCDBitmap* (*copyBitmap)(LCDBitmap* bitmap);
	void (*loadIntoBitmap)(const char* path, LCDBitmap* bitmap, const char** outerr);
	void (*getBitmapData)(LCDBitmap* bitmap, int* width, int* height, int* rowbytes, uint8_t** mask, uint8_t** data);
	void (*clearBitmap)(LCDBitmap* bitmap, LCDColor bgcolor);
	LCDBitmap* (*rotatedBitmap)(LCDBitmap* bitmap, float rotation, float xscale, float yscale, int* allocedSize);

	// LCDBitmapTable
	LCDBitmapTable* (*newBitmapTable)(int count, int width, int height);
	void (*freeBitmapTable)(LCDBitmapTable* table);
	LCDBitmapTable* (*loadBitmapTable)(const char* path, const char** outerr);
	void (*loadIntoBitmapTable)(const char* path, LCDBitmapTable* table, const char** outerr);
	LCDBitmap* (*getTableBitmap)(LCDBitmapTable* table, int idx);

	// LCDFont
	LCDFont* (*loadFont)(const char* path, const char** outErr);
	LCDFontPage* (*getFontPage)(LCDFont* font, uint32_t c);
	LCDFontGlyph* (*getPageGlyph)(LCDFontPage* page, uint32_t c, LCDBitmap** bitmap, int* advance);
	int (*getGlyphKerning)(LCDFontGlyph* glyph, uint32_t glyphcode, uint32_t nextcode);
	int (*getTextWidth)(LCDFont* font, const void* text, size_t len, PDStringEncoding encoding, int tracking);

	// raw framebuffer access
	uint8_t* (*getFrame)(void); // row stride = LCD_ROWSIZE
	uint8_t* (*getDisplayFrame)(void); // row stride = LCD_ROWSIZE
	LCDBitmap* (*getDebugBitmap)(void); // valid in simulator only, function is NULL on device
	LCDBitmap* (*copyFrameBufferBitmap)(void);
	void (*markUpdatedRows)(int start, int end);
	void (*display)(void);

	// misc util.
	void (*setColorToPattern)(LCDColor* color, LCDBitmap* bitmap, int x, int y);
	int (*checkMaskCollision)(LCDBitmap* bitmap1, int x1, int y1, LCDBitmapFlip flip1, LCDBitmap* bitmap2, int x2, int y2, LCDBitmapFlip flip2, LCDRect rect);

	// 1.1
	void (*setScreenClipRect)(int x, int y, int width, int height);

	// 1.1.1
	void (*fillPolygon)(int nPoints, int* coords, LCDColor color, LCDPolygonFillRule fillrule);
	uint8_t (*getFontHeight)(LCDFont* font);

	// 1.7
	LCDBitmap* (*getDisplayBufferBitmap)(void);
	void (*drawRotatedBitmap)(LCDBitmap* bitmap, int x, int y, float rotation, float centerx, float centery, float xscale, float yscale);
	void (*setTextLeading)(int lineHeightAdustment);

	// 1.8
	int (*setBitmapMask)(LCDBitmap* bitmap, LCDBitmap* mask);
	LCDBitmap* (*getBitmapMask)(LCDBitmap* bitmap);

	// 1.10
	void (*setStencilImage)(LCDBitmap* stencil, int tile);

	// 1.12
	LCDFont* (*makeFontFromData)(LCDFontData* data, int wide);
};
*/

#[repr(C)]
pub struct PlaydateGraphics {
	// const struct playdate_video* video;
	pub video: *const PlaydateVideo,




	/*--------------- DRAWING FUNCTIONS ------------------*/
	// void (*clear)(LCDColor color);
	pub clear: extern "C" fn(color: LCDColor),

	// void (*setBackgroundColor)(LCDSolidColor color);
	pub set_background_color: extern "C" fn(color: LCDSolidColor),

	// void (*setStencil)(LCDBitmap* stencil); // deprecated in favor of setStencilImage, which adds a "tile" flag
	#[deprecated(note = "use set_stencil_image")]
	pub set_stencil: extern "C" fn(stencil: *const LCDBitmap),

	// void (*setDrawMode)(LCDBitmapDrawMode mode);
	pub set_draw_mode: extern "C" fn(LCDBitmapDrawMode),

	// void (*setDrawOffset)(int dx, int dy);
	pub set_draw_offset: extern "C" fn (dx: i32, dy: i32),

	// void (*setClipRect)(int x, int y, int width, int height);
	pub set_clip_rect: extern "C" fn(x: i32, y: i32, w: i32, h: i32),

	// void (*clearClipRect)(void);
	pub clear_clip_rect: extern "C" fn(),

	// void (*setLineCapStyle)(LCDLineCapStyle endCapStyle);
	pub set_line_cap_style: extern "C" fn(end_cap_style: LCDLineCapStyle),

	// void (*setFont)(LCDFont* font);
	pub set_font: extern "C" fn(font: *const LCDFont),

	// void (*setTextTracking)(int tracking);
	pub set_text_tracking: extern "C" fn(tracking: i32),

	// void (*pushContext)(LCDBitmap* target);
	pub push_context: extern "C" fn(target: *const LCDBitmap),

	// void (*popContext)(void);
	pub pop_context: extern "C" fn(),



	// void (*drawBitmap)(LCDBitmap* bitmap, int x, int y, LCDBitmapFlip flip);
	pub draw_bitmap: extern "C" fn(bmp: *const LCDBitmap, x: i32, y: i32, flip: LCDBitmapFlip),

	// void (*tileBitmap)(LCDBitmap* bitmap, int x, int y, int width, int height, LCDBitmapFlip flip);
	pub title_bitmap: extern "C" fn(bmp: *const LCDBitmap, x: i32, y: i32, flip: LCDBitmapFlip),


	// void (*drawLine)(int x1, int y1, int x2, int y2, int width, LCDColor color);
	pub draw_line: extern "C" fn(x1: i32, y1: i32, x2: i32, y2: i32, w: i32, color: LCDColor),

	// void (*fillTriangle)(int x1, int y1, int x2, int y2, int x3, int y3, LCDColor color);
	pub fill_triangle: extern "C" fn(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: LCDColor),

	// void (*drawRect)(int x, int y, int width, int height, LCDColor color);
	pub draw_rect: extern "C" fn(x: i32, y: i32, w: i32, h: i32, color: LCDColor),

	// void (*fillRect)(int x, int y, int width, int height, LCDColor color);
	// void (*drawEllipse)(int x, int y, int width, int height, int lineWidth, float startAngle, float endAngle, LCDColor color); // stroked inside the rect
	// void (*fillEllipse)(int x, int y, int width, int height, float startAngle, float endAngle, LCDColor color);
	// void (*drawScaledBitmap)(LCDBitmap* bitmap, int x, int y, float xscale, float yscale);
	// int  (*drawText)(const void* text, size_t len, PDStringEncoding encoding, int x, int y);
}


/*
typedef struct
{
	int left;
	int right; // not inclusive
	int top;
	int bottom; // not inclusive
} LCDRect;

static inline LCDRect LCDMakeRect(int x, int y, int width, int height)
{
	// XXX - assumes width and height are positive
	LCDRect r = { .left = x, .right = x + width, .top = y, .bottom = y + height };
	return r;
}

static inline LCDRect LCDRect_translate(LCDRect r, int dx, int dy)
{
	return (LCDRect){ .left = r.left + dx, .right = r.right + dx, .top = r.top + dy, .bottom = r.bottom + dy };
}

#define LCD_COLUMNS	400
#define LCD_ROWS	240
#define LCD_ROWSIZE 52
#define LCD_SCREEN_RECT LCDMakeRect(0,0,LCD_COLUMNS,LCD_ROWS)

typedef enum
{
	kDrawModeCopy,
	kDrawModeWhiteTransparent,
	kDrawModeBlackTransparent,
	kDrawModeFillWhite,
	kDrawModeFillBlack,
	kDrawModeXOR,
	kDrawModeNXOR,
	kDrawModeInverted
} LCDBitmapDrawMode;
*/

#[repr(C)]
pub enum LCDBitmapDrawMode{
	Copy,
	WhiteTransparent,
	BlackTransparent,
	FillWhite,
	FillBlack,
	XOR,
	NXOR,
	Inverted
}

/*
typedef enum
{
	kBitmapUnflipped,
	kBitmapFlippedX,
	kBitmapFlippedY,
	kBitmapFlippedXY
} LCDBitmapFlip;
*/

#[repr(C)]
pub enum LCDBitmapFlip {
	Unflipped,
	X,
	Y,
	XY
}

/*
typedef enum
{
	kColorBlack,
	kColorWhite,
	kColorClear,
	kColorXOR
} LCDSolidColor;
*/

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum LCDSolidColor {
	Black,
	White,
	Clear,
	XOR
}

/*

typedef enum
{
	kLineCapStyleButt,
	kLineCapStyleSquare,
	kLineCapStyleRound
} LCDLineCapStyle;

*/

#[repr(C)]
pub enum LCDLineCapStyle {
	Butt,
	Square,
	Round
}

/*
typedef enum
{
	kLCDFontLanguageEnglish,
	kLCDFontLanguageJapanese,
	kLCDFontLanguageUnknown,
} LCDFontLanguage;
*/

#[repr(C)]
pub enum LCDFontLanguage {
	English,
	Japanese,
	Unknown,
}

/*
typedef enum
{
	kASCIIEncoding,
	kUTF8Encoding,
	k16BitLEEncoding
} PDStringEncoding;
*/

#[repr(C)]
pub enum PDStringEncoding {
	ASCII,
	UTF8,
	UTF16LE
}

/*
typedef uint8_t LCDPattern[16]; // 8x8 pattern: 8 rows image data, 8 rows mask
typedef uintptr_t LCDColor; // LCDSolidColor or pointer to LCDPattern
*/

pub type LCDPattern = [u8;16];

#[repr(C)]
pub union LCDColor {
	pub color: LCDSolidColor,
	pub pattern: *const LCDPattern,
}

/*
#define LCDMakePattern(r0,r1,r2,r3,r4,r5,r6,r7,r8,r9,ra,rb,rc,rd,re,rf) (LCDPattern){(r0),(r1),(r2),(r3),(r4),(r5),(r6),(f7),(r8),(r9),(ra),(rb),(rc),(rd),(re),(rf)}
#define LCDOpaquePattern(r0,r1,r2,r3,r4,r5,r6,r7) (LCDPattern){(r0),(r1),(r2),(r3),(r4),(r5),(r6),(r7),0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff}

typedef enum
{
	kPolygonFillNonZero,
	kPolygonFillEvenOdd
} LCDPolygonFillRule;

#endif
*/

/*
typedef struct LCDBitmap LCDBitmap;
typedef struct LCDBitmapTable LCDBitmapTable;
typedef struct LCDFont LCDFont;
typedef struct LCDFontData LCDFontData;
typedef struct LCDFontPage LCDFontPage;
typedef struct LCDFontGlyph LCDFontGlyph;
typedef struct LCDVideoPlayer LCDVideoPlayer;
*/

#[repr(C)]
pub struct LCDBitmap {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct LCDBitmapTable {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct LCDFont {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct LCDFontData {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct LCDFontPage {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct LCDFontGlyph {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct LCDVideoPlayer {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/*
struct playdate_video
{
	LCDVideoPlayer* (*loadVideo)(const char* path);
	void (*freePlayer)(LCDVideoPlayer* p);
	int (*setContext)(LCDVideoPlayer* p, LCDBitmap* context);
	void (*useScreenContext)(LCDVideoPlayer* p);
	int (*renderFrame)(LCDVideoPlayer* p, int n);
	const char* (*getError)(LCDVideoPlayer* p);
	void (*getInfo)(LCDVideoPlayer* p, int* outWidth, int* outHeight, float* outFrameRate, int* outFrameCount, int* outCurrentFrame);
	LCDBitmap* (*getContext)(LCDVideoPlayer *p);
};
*/

#[repr(C)]
pub struct PlaydateVideo {
	pub load_video: extern "C" fn(path: *const char) -> *const LCDVideoPlayer,
	pub free_player: extern "C" fn(*const LCDVideoPlayer),
	pub set_context: extern "C" fn(*const LCDVideoPlayer, ctx: *const LCDBitmap) -> i32,
	pub use_screen_context: extern "C" fn(*const LCDVideoPlayer),
	pub render_frame: extern "C" fn(*const LCDVideoPlayer, n: i32) -> i32,
	pub get_error: extern "C" fn(*const LCDVideoPlayer) -> *const char,
	pub get_info: extern "C" fn(
		*const LCDVideoPlayer, 
		out_width: *mut i32,
		out_height: *mut i32,
		out_frame_rate: *mut f32,
		out_frame_count: *mut i32,
		out_current_frame: *mut i32,
	),
	pub get_context: extern "C" fn(*const LCDVideoPlayer) -> *const LCDBitmap,
}

/*

struct playdate_graphics
{
	...
};
*/