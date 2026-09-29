use bmesh::BMEdge;
use bmesh::BMFace;
use bmesh::BMIter;
use bmesh::BMVert;
use bmesh::BMesh;
use bmesh::{BMesh, BM_ITER_MESH, BM_FACES_OF_MESH, BM_EDGES_OF_MESH, BM_elem_flag_test, BM_elem_flag_enable, BM_elem_flag_disable, BM_ITER_LOOP, BM_FACE_FIRST_LOOP, BM_remove_tagged_faces, BM_remove_tagged_edges, BM_remove_tagged_verts};


fn bmo_remove_tagged_faces(bm: &mut BMesh, oflag: short) -> None {
  BMFace::iter(bm, BM_FACES_OF_MESH)
    .for_each(|f| if BMO_face_flag_test(bm, f, oflag) { BM_face_kill(bm, f); })
}

fn bmo_remove_tagged_edges(bm: &mut BMesh, oflag: short) -> None {
  BMEdge::iter(bm, BM_EDGES_OF_MESH)
    .for_each(|e| if BMO_edge_flag_test(bm, e, oflag) { BM_edge_kill(bm, e); })
}

fn bmo_remove_tagged_verts(bm: &mut BMesh, oflag: short) -> None {
  BMVert::iter(bm, BM_VERTS_OF_MESH)
    .for_each(|v| if BMO_vert_flag_test(bm, v, oflag) { BM_vert_kill(bm, v); })
}

fn bmo_remove_tagged_verts_loose(bm: &mut BMesh, oflag: short) -> None {
  BMVert::iter(bm, BM_VERTS_OF_MESH)
    .for_each(|v| if BMO_vert_flag_test(bm, v, oflag) && v->e == nullptr { BM_vert_kill(bm, v); })
}

fn BMO_mesh_delete_oflag_tagged(bm: &mut BMesh, oflag: short, htype: u32) -> None {
  if (htype & BM_FACE) {
    bmo_remove_tagged_faces(bm, oflag);
  }
  if (htype & BM_EDGE) {
    bmo_remove_tagged_edges(bm, oflag);
  }
  if (htype & BM_VERT) {
    bmo_remove_tagged_verts(bm, oflag);
  }
}

fn BMO_mesh_delete_oflag_context(bm: &mut BMesh, oflag: short, type: u32, prepare_fn: fn()) -> None {
  BMEdge::iter(bm, BM_EDGES_OF_MESH)
    .for_each(|e| if BMO_edge_flag_test(bm, e, oflag) {
      BMO_vert_flag_enable(bm, e->v1, oflag);
      BMO_vert_flag_enable(bm, e->v2, oflag);
    })
  .for_each(|e| if BMO_edge_flag_test(bm, e, oflag) {
    BM_edge_kill(bm, e);
  })
  .for_each(|v| if BMO_vert_flag_test(bm, v, oflag) {
    BM_vert_kill(bm, v);
  })
  .for_each(|v| if BMO_vert_flag_test(bm, v, oflag) && v->e == nullptr {
    BM_vert_kill(bm, v);
  })
  if (prepare_fn) { prepare_fn(); }
}

fn bm_remove_tagged_faces(bm: &mut BMesh, hflag: &str) -> None {
    BMIter iter;
    BM_ITER_MESH_MUTABLE(&mut iter, &mut BMFace::of(bm), BM_FACES_OF_MESH) {
        if (BM_elem_flag_test(&mut *iter, hflag)) {
            BM_face_kill(bm, &mut *iter);
        }
    }
}

fn bm_remove_tagged_edges(bm: &mut BMesh, hflag: &str) -> None {
    BMIter iter;
    BM_ITER_MESH_MUTABLE(&mut iter, &mut BMEdge::of(bm), BM_EDGES_OF_MESH) {
        if (BM_elem_flag_test(&mut *iter, hflag)) {
            BM_edge_kill(bm, &mut *iter);
        }
    }
}

fn bm_remove_tagged_verts(bm: &mut BMesh, hflag: &str) -> None {
    BMIter iter;
    BM_ITER_MESH_MUTABLE(&mut iter, &mut BMVert::of(bm), BM_VERTS_OF_MESH) {
        if (BM_elem_flag_test(&mut *iter, hflag)) {
            BM_vert_kill(bm, &mut *iter);
        }
    }
}

fn bm_remove_tagged_verts_loose(bm: &mut BMesh, hflag: &str) -> None {
    BMIter iter;
    BM_ITER_MESH_MUTABLE(&mut iter, &mut BMVert::of(bm), BM_VERTS_OF_MESH) {
        if (BM_elem_flag_test(&mut *iter, hflag) && (&mut *iter->e == nullptr)) {
            BM_vert_kill(bm, &mut *iter);
        }
    }
}

fn BM_mesh_delete_hflag_tagged(bm: &mut BMesh, hflag: &str, htype: &str) -> None {
    if (htype & BM_FACE) {
        bm_remove_tagged_faces(bm, hflag);
    }
    if (htype & BM_EDGE) {
        bm_remove_tagged_edges(bm, hflag);
    }
    if (htype & BM_VERT) {
        bm_remove_tagged_verts(bm, hflag);
    }
}

fn BM_mesh_delete_hflag_context(bm: &mut BMesh, hflag: &str, type: &str) -> None {
    BMIter eiter;
    BMIter fiter;

    match type {
        DEL_VERTS => {
            bm_remove_tagged_verts(bm, hflag);
        },
        DEL_EDGES => {
            BMEdge *e;
            BM_ITER_MESH(&mut e, &mut eiter, bm, BM_EDGES_OF_MESH) {
                if (BM_elem_flag_test(&mut *e, hflag)) {
                    BM_elem_flag_enable(&mut *e->v1, hflag);
                    BM_elem_flag_enable(&mut *e->v2, hflag);
                }
            }
            bm_remove_tagged_edges(bm, hflag);
            bm_remove_tagged_verts_loose(bm, hflag);
        },
        DEL_EDGESFACES => {
            bm_remove_tagged_edges(bm, hflag);
        },
        DEL_ONLYFACES => {
            bm_remove_tagged_faces(bm, hflag);
        },
        DEL_ONLYTAGGED => {
            BM_mesh_delete_hflag_tagged(bm, hflag, BM_ALL_NOLOOP);
        },
    }
}

fn delete_faces(bm: &mut BMesh, hflag: bool) {
    BMFace::iter_mesh(bm, &mut fiter, BM_FACES_OF_MESH) {
        if (BM_elem_flag_test(&mut f, hflag)) {
            BMLoop::iter_loop(&mut fiter, &mut l_first, BM_FACE_FIRST_LOOP) {
                l_first.next_loop()
                    .iter_loop(&mut l_iter, BM_FACE_LOOP) {
                        l_iter.next_loop()
                            .iter_loop(&mut l_next, BM_VERT_LOOP) {
                                l_next.next_loop()
                                    .iter_loop(&mut l_next_next, BM_VERT_LOOP) {
                                        l_next_next.next_loop()
                                            .iter_loop(&mut l_next_next_next, BM_VERT_LOOP) {
                                                l_next_next_next.next_loop()
                                                    .iter_loop(&mut l_next_next_next_next, BM_VERT_LOOP) {
                                                        l_next_next_next_next.next_loop()
                                                            .iter_loop(&mut l_next_next_next_next_next, BM_VERT_LOOP) {
                                                                l_next_next_next_next_next.next_loop()
                                                                    .iter_loop(&mut l_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                        l_next_next_next_next_next_next.next_loop()
                                                                            .iter_loop(&mut l_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                l_next_next_next_next_next_next_next.next_loop()
                                                                                    .iter_loop(&mut l_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                        l_next_next_next_next_next_next_next_next.next_loop()
                                                                                            .iter_loop(&mut l_next_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                                l_next_next_next_next_next_next_next_next_next.next_loop()
                                                                                                    .iter_loop(&mut l_next_next_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                                        l_next_next_next_next_next_next_next_next_next_next.next_loop()
                                                                                                            .iter_loop(&mut l_next_next_next_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                                                l_next_next_next_next_next_next_next_next_next_next_next.next_loop()
                                                                                                                    .iter_loop(&mut l_next_next_next_next_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                                                        l_next_next_next_next_next_next_next_next_next_next_next_next.next_loop()
                                                                                                                            .iter_loop(&mut l_next_next_next_next_next_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                                                                l_next_next_next_next_next_next_next_next_next_next_next_next_next.next_loop()
                                                                                                                                    .iter_loop(&mut l_next_next_next_next_next_next_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                                                                        l_next_next_next_next_next_next_next_next_next_next_next_next_next_next.next_loop()
                                                                                                                                            .iter_loop(&mut l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                                                                                l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next.next_loop()
                                                                                                                                                    .iter_loop(&mut l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                                                                                        l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next.next_loop()
                                                                                                                                                            .iter_loop(&mut l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                                                                                                l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next.next_loop()
                                                                                                                                                                    .iter_loop(&mut l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                                                                                                        l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next.next_loop()
                                                                                                                                                                            .iter_loop(&mut l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                                                                                                                l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next.next_loop()
                                                                                                                                                                                    .iter_loop(&mut l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                                                                                                                        l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next.next_loop()
                                                                                                                                                                                            .iter_loop(&mut l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next, BM_VERT_LOOP) {
                                                                                                                                                                                                l_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next_next.next_loop()
                                                                                                                                                                                                    .iter_loop(&mut l_next_next_next_next_next_next_next_next_next_next_next_next_next
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
