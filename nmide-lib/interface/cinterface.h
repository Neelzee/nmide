#ifndef CINTERFACE

#define CINTERFACE

#include "../html/html.h"
#include "../model/cmodel.h"
#include "../msg/cmsg.h"

CModel init();

CModel update(CMsg msg, CModel model);

CHtmlLocation view(CModel model);

#endif // !CINTERFACE
