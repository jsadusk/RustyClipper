#include <polyclipping/clipper.hpp>
#include <iostream>
#include <exception>

extern "C" {

struct ReturnCodeMsg {
    int code;
    const char *msg;
};

#define SAFE_WRAP(CODE) try { CODE } \
    catch (ClipperLib::clipperException &e) {   \
        return ReturnCodeMsg { 2, e.what() };   \
    } catch (std::exception &e) {          \
        return ReturnCodeMsg { 1, e.what() };   \
    } return ReturnCodeMsg { 0, nullptr };

ReturnCodeMsg path_new(void **obj, ClipperLib::IntPoint **data) {
    SAFE_WRAP(
        ClipperLib::Path *newPath = new ClipperLib::Path();
        *obj = (void*)newPath;
        *data = newPath->data();
    )
}

ReturnCodeMsg path_data(const void *obj, ClipperLib::IntPoint **data, size_t *size) {
    SAFE_WRAP(
        ClipperLib::Path &path = *(ClipperLib::Path*)obj;
        *data = path.data();
        *size = path.size();
    )
}

ReturnCodeMsg path_new_sized(size_t size, void **obj, ClipperLib::IntPoint **data) {
    SAFE_WRAP(
        ClipperLib::Path *newPath = new ClipperLib::Path(size);
        *obj = (void*)newPath;
        *data = newPath->data();
    )
}

ReturnCodeMsg path_push_back(void *obj, const ClipperLib::IntPoint *elem,
                   ClipperLib::IntPoint **data) {
    SAFE_WRAP(
        ClipperLib::Path &path = *(ClipperLib::Path*)obj;
        path.push_back(*elem);
        *data = path.data();
    )
}

void path_delete(void *obj) {
    ClipperLib::Path *path = (ClipperLib::Path*)obj;
    delete path;
}

ReturnCodeMsg paths_new(void **obj, void **data) {
    SAFE_WRAP(
        ClipperLib::Paths *newPaths = new ClipperLib::Paths();
        *obj = (void*)newPaths;
        *data = newPaths->data();
    )
}


ReturnCodeMsg paths_new_sized(size_t size, void **obj, void **data) {
    SAFE_WRAP(
        ClipperLib::Paths *newPaths = new ClipperLib::Paths(size);
        *obj = (void*)newPaths;
        *data = newPaths->data();
    )
}

ReturnCodeMsg paths_data(const void *obj, void **data, size_t *size) {
    SAFE_WRAP(
        ClipperLib::Paths &paths = *(ClipperLib::Paths*)obj;
        *data = paths.data();
        *size = paths.size();
    )
}

ReturnCodeMsg paths_push_back_move(void *obj, void *elem, void **data) {
    SAFE_WRAP(
        ClipperLib::Paths &paths = *(ClipperLib::Paths*)obj;
        ClipperLib::Path &path = *(ClipperLib::Path*)elem;
        paths.push_back(std::move(path));
        *data = paths.data();
    )
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

ReturnCodeMsg clipper_new(void **obj, int initOptions) {
    SAFE_WRAP(
        ClipperLib::Clipper *newclipper = new ClipperLib::Clipper(initOptions);
        *obj = (void*)newclipper;
    )
}

ReturnCodeMsg clipper_add_path(void *clipper_obj, const void *path_obj,
                               const ClipperLib::PolyType poly_type,
                               int closed) {
    SAFE_WRAP(
        ClipperLib::Clipper *clipper = (ClipperLib::Clipper*)clipper_obj;
        const ClipperLib::Path *path = (ClipperLib::Path*)path_obj;
        clipper->AddPath(*path, poly_type, closed);
    )
}

ReturnCodeMsg clipper_add_paths(void *clipper_obj, const void *paths_obj,
                      const ClipperLib::PolyType poly_type,
                      int closed) {
    SAFE_WRAP(
        ClipperLib::Clipper *clipper = (ClipperLib::Clipper*)clipper_obj;
        const ClipperLib::Paths *paths = (ClipperLib::Paths*)paths_obj;
        clipper->AddPaths(*paths, poly_type, closed);
    )
}

ReturnCodeMsg clipper_offset_add_path(void *clipper_offset_obj, const void *path_obj,
                            const ClipperLib::JoinType join_type,
                            const ClipperLib::EndType end_type) {
    SAFE_WRAP(
        ClipperLib::ClipperOffset *clipper_offset = (ClipperLib::ClipperOffset*)clipper_offset_obj;
        const ClipperLib::Path *path = (ClipperLib::Path*)path_obj;
        clipper_offset->AddPath(*path, join_type, end_type);
    )
}

ReturnCodeMsg clipper_offset_add_paths(void *clipper_offset_obj, const void *paths_obj,
                             const ClipperLib::JoinType join_type,
                             const ClipperLib::EndType end_type) {
    SAFE_WRAP(
        ClipperLib::ClipperOffset *clipper_offset = (ClipperLib::ClipperOffset*)clipper_offset_obj;
        const ClipperLib::Paths *paths = (ClipperLib::Paths*)paths_obj;
        clipper_offset->AddPaths(*paths, join_type, end_type);
    )
}

ReturnCodeMsg clipper_execute_open_closed(
      void *clipper_obj, void **solution_open_obj, void **solution_closed_obj,
      const ClipperLib::ClipType clip_type,
      const ClipperLib::PolyFillType subj_fill_type,
      const ClipperLib::PolyFillType clip_fill_type) {
    SAFE_WRAP(
        ClipperLib::Clipper *clipper = (ClipperLib::Clipper*)clipper_obj;
        ClipperLib::PolyTree solution;
        clipper->Execute(clip_type, solution, subj_fill_type, clip_fill_type);
        ClipperLib::Paths *solution_open = new ClipperLib::Paths();
        ClipperLib::OpenPathsFromPolyTree(solution, *solution_open);
        ClipperLib::Paths *solution_closed = new ClipperLib::Paths();
        ClipperLib::ClosedPathsFromPolyTree(solution, *solution_closed);
        *solution_open_obj = (void*)solution_open;
        *solution_closed_obj = (void*)solution_closed;
    )
}

ReturnCodeMsg clipper_delete(void *obj) {
    ClipperLib::Clipper *clipper = (ClipperLib::Clipper*)obj;
    delete clipper;
}

ReturnCodeMsg clipper_offset_new(void **obj) {
    SAFE_WRAP(
        ClipperLib::ClipperOffset *newClipperOffset =
            new ClipperLib::ClipperOffset();
        *obj = (void*)newClipperOffset;
    )
}

ReturnCodeMsg clipper_offset_delete(void **obj) {
    ClipperLib::ClipperOffset *clipperOffset = (ClipperLib::ClipperOffset*)obj;
    delete clipperOffset;
}

}
