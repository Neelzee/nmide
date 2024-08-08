#ifndef HTML_LIB

#define HTML_LIB

#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

/**
 * Enumeration of HTML-Tags
 **/
typedef enum CHtmlTag {
  Div,
  P,
  Span,
  Section,
  Input,
  Button,
  Script,
  Select,
  Aside,
  Nav,
  A,
  None,
} CHtmlTag;

/**
 * Raw text, used if you want a raw string in another node.
 */
typedef struct CHtmlText {
  /**
   * Text field
   */
  char *text;
  /**
   * Amount of characters.
   * TODO: Ensure this is safe with UTF-8
   */
  size_t len;
} CHtmlText;

/**
 * Standard Html Element Representation.
 *
 * Represents any Html-tag, along with their subsequent children.
 */
typedef struct CHtmlElement {
  /**
   * Kind of Html element
   */
  enum CHtmlTag tag;
  /**
   * Array of children
   */
  struct CHtml **children;
  /**
   * Amount of children
   */
  size_t len;
} CHtmlElement;

/**
 * Union of a Html Element, and Raw text.
 */
typedef union CHtmlContent {
  CHtmlElement *element;
  CHtmlText *text;
} CHtmlContent;

typedef struct CHtml {
  CHtmlContent *content;
  bool isElement;
} CHtml;

typedef struct CHtmlLocation {
  CHtml html;
  char *location;
} CHtmlLocation;

/**
 * Creates empty CHtmlElement Div
 */
CHtmlElement *e_div();

/**
 * Creates empty CHtmlText text
 */
CHtmlText *e_text();

CHtmlContent *unionize(CHtmlElement *element, CHtmlText *text);

/**
 * <div>
 *  <p>Hello, World!</p>
 * </div>
 */
CHtml *simple_test();

/**
 * Recursively frees the given html node
 * TODO: Find out if this works.
 */
void free_chtml(CHtml *chtml);

#endif // !HTML_LIB
