#include <sys/param.h>
#include <sys/queue.h>
#include <fs/devfs/devfs.h>
#include <elf-hints.h>

// TODO remove usage once bindgen supports proper #define constant handling
const int int_DRA_BACTS = DRA_BACTS;
const int int_DRC_PATHPTRN = DRC_PATHPTRN;
const int int_DRB_HIDE = DRB_HIDE;
const int int_DRB_UNHIDE = DRB_UNHIDE;
