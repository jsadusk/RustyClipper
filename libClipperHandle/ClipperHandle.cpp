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

int path_data(const void *obj, ClipperLib::IntPoint **data) {
    try {
        ClipperLib::Path &path = *(ClipperLib::Path*)obj;
        *data = path.data();
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

int paths_new(void **obj, void **data) {
    try {
        ClipperLib::Paths *newPaths = new ClipperLib::Paths();
        *obj = (void*)newPaths;
        *data = newPaths->data();
    } catch (...) {
        return 1;
    }

    return 0;
}


int paths_new_sized(size_t size, void **obj, void **data) {
    try {
        ClipperLib::Paths *newPaths = new ClipperLib::Paths(size);
        *obj = (void*)newPaths;
        *data = newPaths->data();
    } catch (...) {
        return 1;
    }

    return 0;
}

int paths_data(const void *obj, void **data) {
    try {
        ClipperLib::Paths &paths = *(ClipperLib::Paths*)obj;
        *data = paths.data();
    } catch (...) {
        return 1;
    }
}

int paths_push_back_move(void *obj, void *elem, void **data) {
    try {
        ClipperLib::Paths &paths = *(ClipperLib::Paths*)obj;
        ClipperLib::Path &path = *(ClipperLib::Path*)elem;
        paths.push_back(std::move(path));
        *data = paths.data();
    } catch (...) {
        return 1;
    }

    return 0;
}

void paths_delete(void *obj) {
    ClipperLib::Paths *paths = (ClipperLib::Paths*)obj;
    delete paths;
}

int test_elem(void *obj, size_t index, long test_x, long test_y) {
    ClipperLib::Path &path = *(ClipperLib::Path*)obj;
    ClipperLib::IntPoint &elem = path[index];

    return elem.X == test_x && elem.Y == test_y;
}

int clipper_new(void **obj, int initOptions) {
    try {
        ClipperLib::Clipper *newclipper = new ClipperLib::Clipper(initOptions);
        *obj = (void*)newclipper;
    } catch (...) {
        return 1;
    }
    
    return 0;
}

int clipper_add_path(void *clipper_obj, const void *path_obj,
                     const ClipperLib::PolyType poly_type,
                     int closed) {
    try {
        ClipperLib::Clipper *clipper = (ClipperLib::Clipper*)clipper_obj;
        const ClipperLib::Path *path = (ClipperLib::Path*)path_obj;
        clipper->AddPath(*path, poly_type, closed);
    } catch (...) {
        return 1;
    }

    return 0;
}

int clipper_offset_add_path(void *clipper_offset_obj, const void *path_obj,
                            const ClipperLib::JoinType join_type,
                            const ClipperLib::EndType end_type) {
    try {
        ClipperLib::ClipperOffset *clipper_offset = (ClipperLib::ClipperOffset*)clipper_offset_obj;
        const ClipperLib::Path *path = (ClipperLib::Path*)path_obj;
        clipper_offset->AddPath(*path, join_type, end_type);
    } catch (...) {
        return 1;
    }

    return 0;
}

int clipper_execute_open_closed(
      void *clipper_obj, void **solution_open_obj, void **solution_closed_obj,
      const ClipperLib::ClipType clip_type,
      const ClipperLib::PolyFillType subj_fill_type,
      const ClipperLib::PolyFillType clip_fill_type) {
    try {
        ClipperLib::Clipper *clipper = (ClipperLib::Clipper*)clipper_obj;
        ClipperLib::PolyTree solution;
        clipper->Execute(clip_type, solution, subj_fill_type, clip_fill_type);
        ClipperLib::Paths *solution_open = new ClipperLib::Paths();
        ClipperLib::OpenPathsFromPolyTree(solution, *solution_open);
        ClipperLib::Paths *solution_closed = new ClipperLib::Paths();
        ClipperLib::ClosedPathsFromPolyTree(solution, *solution_closed);
        *solution_open_obj = (void*)solution_open;
        *solution_closed_obj = (void*)solution_closed;
    } catch (...) {
        return 1;
    }

    return 0;
}

int clipper_delete(void *obj) {
    ClipperLib::Clipper *clipper = (ClipperLib::Clipper*)obj;
    delete clipper;
}

int clipper_offset_new(void **obj) {
    try {
        ClipperLib::ClipperOffset *newClipperOffset =
            new ClipperLib::ClipperOffset();
        *obj = (void*)newClipperOffset;
    } catch (...) {
        return 1;
    }

    return 0;
}

int clipper_offset_delete(void **obj) {
    ClipperLib::ClipperOffset *clipperOffset = (ClipperLib::ClipperOffset*)obj;
    delete clipperOffset;
}

}
