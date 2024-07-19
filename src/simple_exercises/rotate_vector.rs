//Todo receive number k from user

fn rotator_manually(nums: &mut Vec<i32>, k: i32) {

    let mut k = k as usize;

    if k > nums.len() {

        k = k % nums.len();
    }

    let mut result = vec![0; nums.len()];

    for i in 0..nums.len() {

        result[i] = nums[(nums.len() - k + i) % nums.len()];
    }

    for i in 0..nums.len() {
        nums[i] = result[i];
    }
}

fn rotator_with_rusts_rotate(nums: &mut Vec<i32>, k: i32) {

    let n = nums.len();

    let k = k as usize % n;

    nums.reverse();
    nums[0..k].reverse();
    nums[k..].reverse();
}

pub fn make_rotator(rotator_type: String) -> Vec<i32> {

        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];

        let k = 3;

        if (rotator_type == "manually") {

            rotator_manually(&mut nums, k);

        } else {

            rotator_with_rusts_rotate(&mut nums, k);
        }

        return nums;
}