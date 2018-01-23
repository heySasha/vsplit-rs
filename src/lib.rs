pub fn split_vec(v: &Vec<u8>, sep: &Vec<u8>, res: &mut Vec<Vec<u8>>) {

    let len_sep = sep.len();
    let len_v = v.len();

    let mut count: usize = 0;
    let mut prev_i: usize = 0;

    for i in 0..len_v {
        if i == prev_i {
            continue;
        }

        if v[i] == sep[count] {
            count += 1;

            if count == len_sep {

                res.push(Vec::from (&v[prev_i..i-len_sep+1]) );

                prev_i = i + 1;
                count = 0;
            }
        } else {
            count = 0;
        }
    }

    if prev_i != len_v {
        res.push(Vec::from (&v[prev_i..len_v]) );
    }
}
