#ifndef CHTML_LIB

#define CHTML_LIB

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
  struct CHtml *children;
  /**
   * Amount of children
   */
  size_t len;
} CHtmlElement;

/**
 * Union of a Html Element, and Raw text.
 */
typedef union CHtmlContent {
  CHtmlElement element;
  const char *text;
} CHtmlContent;

typedef struct CHtml {
  CHtmlContent content;
  bool isElement;
} CHtml;

typedef struct CHtmlLocation {
  CHtml html;
  const char location;
} CHtmlLocation;

/**
 * Creates empty CHtmlElement Div
 */
CHtmlElement *e_div();

CHtmlContent *unionize(CHtmlElement *element, char *text);

/**
 * <div>
 *  <p>Hello, World!</p>
 * </div>
 */
CHtml simple_test();

/**
 * Recursively frees the given html node
 * TODO: Find out if this works.
 */
void free_chtml(CHtml *chtml);

#endif // !CHTML_LIB
