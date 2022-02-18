use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct MemberItem {
    pub neg_1: i32,
    pub neg_2: i32,
    pub neg_3: i32,
    pub pos_1: i32,
    pub pos_2: i32,
    pub pos_3: i32,
    pub unknown_1: i32,
    pub item_type: i32,
    pub unknown_2: i32,
    pub length: i32,
    pub start: i32,
    pub negative: i32,
    pub unknown_3: i32,
    pub k: i32,
    pub l: i32,
    pub m: i32,
    pub n: i32,
    pub o: i32,
    pub p: i32,
    pub q: i32,
    pub r: i32,
    pub s: i32,
    pub t: i32,
    pub u: i32,
    pub v: i32,
    pub w: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl MemberItem {
#[allow(clippy::too_many_arguments)]
pub fn new_t1(neg1: i32, neg2: i32, pos1: i32, pos2: i32, na1: i32, item_type: i32, na2: i32, length: i32, start: i32, q: i32, r: i32, s: i32, t: i32, u: i32, v: i32, w: i32, x: i32, y: i32, z: i32) -> MemberItem {
    MemberItem {
        neg_1: neg1,
        neg_2: neg2,
        pos_1: pos1,
        pos_2: pos2,
        unknown_1: na1,
        item_type,
        unknown_2: na2,
        length,
        start,
        q,
        r,
        s,
        t,
        u,
        v,
        w,
        x,
        negative: y,
        unknown_3: z,
        ..Default::default()
    }
}

#[allow(clippy::too_many_arguments)]
pub fn new_t3(neg1: i32, neg2: i32, neg3: i32, pos1: i32, pos2: i32, pos3: i32, item_type: i32, na1: i32, length: i32, na2: i32, start: i32, m: i32, n: i32, o: i32, p: i32, q: i32, r: i32, s: i32, t: i32, u: i32, v: i32, w: i32, x: i32, y: i32, z: i32) -> MemberItem {
    MemberItem {
        neg_1: neg1,
        neg_2: neg2,
        neg_3: neg3,
        pos_1: pos1,
        pos_2: pos2,
        pos_3: pos3,
        item_type,
        unknown_1: na1,
        length,
        unknown_2: start,
        start: na2,
        m,
        n,
        o,
        p,
        q,
        r,
        s,
        negative: r,
        unknown_3: s,
        t,
        u,
        v,
        w,
        x,
        y,
        z,
        ..Default::default()
    }
}

#[allow(clippy::too_many_arguments)]
pub fn new_t2(neg1: i32, neg2: i32, neg3: i32, pos1: i32, pos2: i32, pos3: i32, item_type: i32, na1: i32, length: i32, na2: i32, start: i32, k: i32, l: i32, m: i32, n: i32, o: i32, p: i32, q: i32, r: i32, s: i32, t: i32, u: i32, v: i32, w: i32, x: i32, y: i32, z: i32) -> MemberItem {
    MemberItem {
        neg_1: neg1,
		neg_2: neg2,
		neg_3: neg3,
		pos_1: pos1,
		pos_2: pos2,
		pos_3: pos3,
		item_type,
		unknown_1: na1,
		length,
		unknown_2: na2,
		start,
		k,
		l,
		m,
		n,
		o,
		p,
		q,
		negative: r,
		unknown_3: s,
		t,
		u,
		v,
		w,
		x,
		y,
		z,
        ..Default::default()
        }
    }
}