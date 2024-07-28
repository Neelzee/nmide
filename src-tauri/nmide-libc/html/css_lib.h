#ifndef CSS_LIB

#include "../nmidelib.h"

/**
 * Enumeration of different HTML Attributes
 **/
typedef enum CAttribute {
  Style,
  Alt,
  Src,
  Href,
  OnClick,
  OnHover,
  OnLeave,
  OnEnter,
} CAttribute;

typedef enum CStyle {
  Width,
  Height,
  Padding,
  PaddingTop,
  PaddingBottom,
  PaddingRight,
  PaddingLeft,
  Margin,
  MarginTop,
  MarginBottom,
  MarginRight,
  MarginLeft,
  BackgroundColor,
} CStyle;

typedef struct CColor {
  unsigned char R;
  unsigned char G;
  unsigned char B;
} CColor;

/**
 * Creates a new CColor, with capped RGB values
 **/
CColor CColorNew(int r, int g, int b);

typedef struct CAttr {
  CAttribute key;
  CString value;
} CAttr;

typedef enum CStyleUnit {
  Pixel,
  REM,
  Percent,
} CStyleUnit;

/**
 * CBoxProperty are all styles pertaining to spacing of CSS elements.
 *
 * Example: Width, Height, Padding-*, Margin-*
 **/
typedef struct CBoxProperty {
  CStyleUnit unit;
  double value;
} CWidth;

#endif // !CSS_LIB
