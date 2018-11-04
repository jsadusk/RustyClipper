#include <polyclipping/clipper.hpp>
#include <iostream>

extern "C" {

int path_new(void **obj, ClipperLib::IntPoint **data) {
    try {
        ClipperLib::Path *newPath = new ClipperLib::Path();
        *obj = (void*)newPath;
        *data = newPath->data();
    } catch (...) {
        return 1;
    }

    return 0;
}


int path_new_sized(size_t size, void **obj, ClipperLib::IntPoint **data) {
    try {
        ClipperLib::Path *newPath = new ClipperLib::Path(size);
        *obj = (void*)newPath;
        *data = newPath->data();
    } catch (...) {
        return 1;
    }

    return 0;
}

int path_push_back(void *obj, const ClipperLib::IntPoint *elem,
                   ClipperLib::IntPoint **data) {
    try {
        ClipperLib::Path &path = *(ClipperLib::Path*)obj;
        path.push_back(*elem);
        *data = path.data();
    } catch (...) {
        return 1;
    }

    return 0;
}

void path_delete(void *obj) {
    ClipperLib::Path *path = (ClipperLib::Path*)obj;
    delete path;
}

int test_elem(void *obj, size_t index, long test_x, long test_y) {
    ClipperLib::Path &path = *(ClipperLib::Path*)obj;
    ClipperLib::IntPoint &elem = path[index];

    return elem.X == test_x && elem.Y == test_y;
}

}
