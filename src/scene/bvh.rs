use crate::scene::material::Material;
use crate::scene::triangle::Triangle;
use crate::util::aabb::Aabb;
use crate::util::hit::Hit;
use crate::util::ray::Ray;
use crate::util::vector::Vec3;

const MAX_LEAF: usize = 4;

#[derive(Debug, Clone, Copy)]
enum Node {
    Leaf {
        bbox: Aabb,
        start: u32,
        end: u32,
    },
    Internal {
        bbox: Aabb,
        left: u32,
        right: u32,
    },
}

impl Node {
    fn bbox(&self) -> Aabb {
        match self {
            Node::Leaf { bbox, .. } | Node::Internal { bbox, .. } => *bbox,
        }
    }
}

pub struct Bvh {
    nodes: Vec<Node>,
    indices: Vec<u32>,
    root: u32,
}

impl Bvh {
    pub fn build(triangles: &[Triangle]) -> Self {
        if triangles.is_empty() {
            return Self {
                nodes: vec![Node::Leaf {
                    bbox: Aabb::empty(),
                    start: 0,
                    end: 0,
                }],
                indices: vec![],
                root: 0,
            };
        }

        let centroids: Vec<Vec3> = triangles
            .iter()
            .map(|t| (t.point_a + t.point_b + t.point_c) * (1.0 / 3.0))
            .collect();
        let tri_bboxes: Vec<Aabb> = triangles
            .iter()
            .map(|t| Aabb::from_triangle(t.point_a, t.point_b, t.point_c))
            .collect();

        let mut indices: Vec<u32> = (0..triangles.len() as u32).collect();
        let mut nodes: Vec<Node> = Vec::with_capacity(triangles.len() * 2 / MAX_LEAF + 1);

        let root = build_recursive(
            &tri_bboxes,
            &centroids,
            &mut indices,
            0,
            triangles.len(),
            &mut nodes,
        );

        Self {
            nodes,
            indices,
            root,
        }
    }

    pub fn closest_hit(&self, ray: &Ray, triangles: &[Triangle]) -> Option<Hit> {
        if triangles.is_empty() {
            return None;
        }
        let mut best: Option<Hit> = None;
        self.closest_hit_rec(self.root, ray, triangles, &mut best);
        best
    }

    fn closest_hit_rec(
        &self,
        idx: u32,
        ray: &Ray,
        triangles: &[Triangle],
        best: &mut Option<Hit>,
    ) {
        let node = self.nodes[idx as usize];
        let max_t = best.as_ref().map(|h| h.lambda).unwrap_or(f32::INFINITY);
        match node.bbox().intersect(ray) {
            Some((t_near, _)) if t_near <= max_t => {}
            _ => return,
        }

        match node {
            Node::Leaf { start, end, .. } => {
                for i in start..end {
                    let tri_idx = self.indices[i as usize] as usize;
                    if let Some(hit) = triangles[tri_idx].get_hit(ray) {
                        let cur = best.as_ref().map(|h| h.lambda).unwrap_or(f32::INFINITY);
                        if hit.lambda > 0.00001 && hit.lambda < cur {
                            *best = Some(hit);
                        }
                    }
                }
            }
            Node::Internal { left, right, .. } => {
                let left_t = self.nodes[left as usize].bbox().intersect(ray).map(|p| p.0);
                let right_t = self.nodes[right as usize].bbox().intersect(ray).map(|p| p.0);
                match (left_t, right_t) {
                    (Some(lt), Some(rt)) if lt <= rt => {
                        self.closest_hit_rec(left, ray, triangles, best);
                        self.closest_hit_rec(right, ray, triangles, best);
                    }
                    (Some(_), Some(_)) => {
                        self.closest_hit_rec(right, ray, triangles, best);
                        self.closest_hit_rec(left, ray, triangles, best);
                    }
                    (Some(_), None) => self.closest_hit_rec(left, ray, triangles, best),
                    (None, Some(_)) => self.closest_hit_rec(right, ray, triangles, best),
                    (None, None) => {}
                }
            }
        }
    }

    pub fn any_blocking_hit(&self, ray: &Ray, triangles: &[Triangle], max_t: f32) -> bool {
        if triangles.is_empty() {
            return false;
        }
        self.any_blocking_rec(self.root, ray, triangles, max_t)
    }

    fn any_blocking_rec(
        &self,
        idx: u32,
        ray: &Ray,
        triangles: &[Triangle],
        max_t: f32,
    ) -> bool {
        let node = self.nodes[idx as usize];
        match node.bbox().intersect(ray) {
            Some((t_near, _)) if t_near <= max_t => {}
            _ => return false,
        }

        match node {
            Node::Leaf { start, end, .. } => {
                for i in start..end {
                    let tri_idx = self.indices[i as usize] as usize;
                    let tri = &triangles[tri_idx];
                    if matches!(tri.material, Material::Dielectric { .. }) {
                        continue;
                    }
                    if let Some(hit) = tri.get_hit(ray) {
                        if hit.lambda > 0.0001 && hit.lambda <= max_t {
                            return true;
                        }
                    }
                }
                false
            }
            Node::Internal { left, right, .. } => {
                self.any_blocking_rec(left, ray, triangles, max_t)
                    || self.any_blocking_rec(right, ray, triangles, max_t)
            }
        }
    }
}

fn build_recursive(
    tri_bboxes: &[Aabb],
    centroids: &[Vec3],
    indices: &mut [u32],
    start: usize,
    end: usize,
    nodes: &mut Vec<Node>,
) -> u32 {
    let mut bbox = Aabb::empty();
    let mut centroid_bbox = Aabb::empty();
    for &tri_idx in &indices[start..end] {
        bbox = Aabb::union(bbox, tri_bboxes[tri_idx as usize]);
        centroid_bbox.extend_point(centroids[tri_idx as usize]);
    }

    let count = end - start;
    if count <= MAX_LEAF {
        let node_idx = nodes.len() as u32;
        nodes.push(Node::Leaf {
            bbox,
            start: start as u32,
            end: end as u32,
        });
        return node_idx;
    }

    let axis = centroid_bbox.longest_axis();
    let split = (axis_get(centroid_bbox.min, axis) + axis_get(centroid_bbox.max, axis)) * 0.5;

    let mut i = start;
    let mut j = end;
    while i < j {
        if axis_get(centroids[indices[i] as usize], axis) < split {
            i += 1;
        } else {
            j -= 1;
            indices.swap(i, j);
        }
    }
    let mut mid = i;

    if mid == start || mid == end {
        mid = start + count / 2;
        indices[start..end].sort_unstable_by(|&a, &b| {
            let av = axis_get(centroids[a as usize], axis);
            let bv = axis_get(centroids[b as usize], axis);
            av.partial_cmp(&bv).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    let node_idx = nodes.len() as u32;
    nodes.push(Node::Leaf {
        bbox: Aabb::empty(),
        start: 0,
        end: 0,
    });
    let left = build_recursive(tri_bboxes, centroids, indices, start, mid, nodes);
    let right = build_recursive(tri_bboxes, centroids, indices, mid, end, nodes);
    nodes[node_idx as usize] = Node::Internal { bbox, left, right };
    node_idx
}

fn axis_get(v: Vec3, axis: usize) -> f32 {
    match axis {
        0 => v.x,
        1 => v.y,
        _ => v.z,
    }
}
