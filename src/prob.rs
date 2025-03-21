const ROLL_1_PROB: [([Die; 1], ExpectedValue); 6] = [
    ([1], ExpectedValue::new_raw(1, 6)),
    ([2], ExpectedValue::new_raw(1, 6)),
    ([3], ExpectedValue::new_raw(1, 6)),
    ([4], ExpectedValue::new_raw(1, 6)),
    ([5], ExpectedValue::new_raw(1, 6)),
    ([6], ExpectedValue::new_raw(1, 6)),
];

const ROLL_2_PROB: [([Die; 2], ExpectedValue); 21] = [
    ([1, 1], ExpectedValue::new_raw(1, 36)),
    ([1, 2], ExpectedValue::new_raw(1, 18)),
    ([1, 3], ExpectedValue::new_raw(1, 18)),
    ([1, 4], ExpectedValue::new_raw(1, 18)),
    ([1, 5], ExpectedValue::new_raw(1, 18)),
    ([1, 6], ExpectedValue::new_raw(1, 18)),
    ([2, 2], ExpectedValue::new_raw(1, 36)),
    ([2, 3], ExpectedValue::new_raw(1, 18)),
    ([2, 4], ExpectedValue::new_raw(1, 18)),
    ([2, 5], ExpectedValue::new_raw(1, 18)),
    ([2, 6], ExpectedValue::new_raw(1, 18)),
    ([3, 3], ExpectedValue::new_raw(1, 36)),
    ([3, 4], ExpectedValue::new_raw(1, 18)),
    ([3, 5], ExpectedValue::new_raw(1, 18)),
    ([3, 6], ExpectedValue::new_raw(1, 18)),
    ([4, 4], ExpectedValue::new_raw(1, 36)),
    ([4, 5], ExpectedValue::new_raw(1, 18)),
    ([4, 6], ExpectedValue::new_raw(1, 18)),
    ([5, 5], ExpectedValue::new_raw(1, 36)),
    ([5, 6], ExpectedValue::new_raw(1, 18)),
    ([6, 6], ExpectedValue::new_raw(1, 36)),
];

const ROLL_3_PROB: [([Die; 3], ExpectedValue); 56] = [
    ([1, 1, 1], ExpectedValue::new_raw(1, 216)),
    ([1, 1, 2], ExpectedValue::new_raw(1, 72)),
    ([1, 1, 3], ExpectedValue::new_raw(1, 72)),
    ([1, 1, 4], ExpectedValue::new_raw(1, 72)),
    ([1, 1, 5], ExpectedValue::new_raw(1, 72)),
    ([1, 1, 6], ExpectedValue::new_raw(1, 72)),
    ([1, 2, 2], ExpectedValue::new_raw(1, 72)),
    ([1, 2, 3], ExpectedValue::new_raw(1, 36)),
    ([1, 2, 4], ExpectedValue::new_raw(1, 36)),
    ([1, 2, 5], ExpectedValue::new_raw(1, 36)),
    ([1, 2, 6], ExpectedValue::new_raw(1, 36)),
    ([1, 3, 3], ExpectedValue::new_raw(1, 72)),
    ([1, 3, 4], ExpectedValue::new_raw(1, 36)),
    ([1, 3, 5], ExpectedValue::new_raw(1, 36)),
    ([1, 3, 6], ExpectedValue::new_raw(1, 36)),
    ([1, 4, 4], ExpectedValue::new_raw(1, 72)),
    ([1, 4, 5], ExpectedValue::new_raw(1, 36)),
    ([1, 4, 6], ExpectedValue::new_raw(1, 36)),
    ([1, 5, 5], ExpectedValue::new_raw(1, 72)),
    ([1, 5, 6], ExpectedValue::new_raw(1, 36)),
    ([1, 6, 6], ExpectedValue::new_raw(1, 72)),
    ([2, 2, 2], ExpectedValue::new_raw(1, 216)),
    ([2, 2, 3], ExpectedValue::new_raw(1, 72)),
    ([2, 2, 4], ExpectedValue::new_raw(1, 72)),
    ([2, 2, 5], ExpectedValue::new_raw(1, 72)),
    ([2, 2, 6], ExpectedValue::new_raw(1, 72)),
    ([2, 3, 3], ExpectedValue::new_raw(1, 72)),
    ([2, 3, 4], ExpectedValue::new_raw(1, 36)),
    ([2, 3, 5], ExpectedValue::new_raw(1, 36)),
    ([2, 3, 6], ExpectedValue::new_raw(1, 36)),
    ([2, 4, 4], ExpectedValue::new_raw(1, 72)),
    ([2, 4, 5], ExpectedValue::new_raw(1, 36)),
    ([2, 4, 6], ExpectedValue::new_raw(1, 36)),
    ([2, 5, 5], ExpectedValue::new_raw(1, 72)),
    ([2, 5, 6], ExpectedValue::new_raw(1, 36)),
    ([2, 6, 6], ExpectedValue::new_raw(1, 72)),
    ([3, 3, 3], ExpectedValue::new_raw(1, 216)),
    ([3, 3, 4], ExpectedValue::new_raw(1, 72)),
    ([3, 3, 5], ExpectedValue::new_raw(1, 72)),
    ([3, 3, 6], ExpectedValue::new_raw(1, 72)),
    ([3, 4, 4], ExpectedValue::new_raw(1, 72)),
    ([3, 4, 5], ExpectedValue::new_raw(1, 36)),
    ([3, 4, 6], ExpectedValue::new_raw(1, 36)),
    ([3, 5, 5], ExpectedValue::new_raw(1, 72)),
    ([3, 5, 6], ExpectedValue::new_raw(1, 36)),
    ([3, 6, 6], ExpectedValue::new_raw(1, 72)),
    ([4, 4, 4], ExpectedValue::new_raw(1, 216)),
    ([4, 4, 5], ExpectedValue::new_raw(1, 72)),
    ([4, 4, 6], ExpectedValue::new_raw(1, 72)),
    ([4, 5, 5], ExpectedValue::new_raw(1, 72)),
    ([4, 5, 6], ExpectedValue::new_raw(1, 36)),
    ([4, 6, 6], ExpectedValue::new_raw(1, 72)),
    ([5, 5, 5], ExpectedValue::new_raw(1, 216)),
    ([5, 5, 6], ExpectedValue::new_raw(1, 72)),
    ([5, 6, 6], ExpectedValue::new_raw(1, 72)),
    ([6, 6, 6], ExpectedValue::new_raw(1, 216)),
];

const ROLL_4_PROB: [([Die; 4], ExpectedValue); 126] = [
    ([1, 1, 1, 1], ExpectedValue::new_raw(1, 1296)),
    ([1, 1, 1, 2], ExpectedValue::new_raw(1, 324)),
    ([1, 1, 1, 3], ExpectedValue::new_raw(1, 324)),
    ([1, 1, 1, 4], ExpectedValue::new_raw(1, 324)),
    ([1, 1, 1, 5], ExpectedValue::new_raw(1, 324)),
    ([1, 1, 1, 6], ExpectedValue::new_raw(1, 324)),
    ([1, 1, 2, 2], ExpectedValue::new_raw(1, 216)),
    ([1, 1, 2, 3], ExpectedValue::new_raw(1, 108)),
    ([1, 1, 2, 4], ExpectedValue::new_raw(1, 108)),
    ([1, 1, 2, 5], ExpectedValue::new_raw(1, 108)),
    ([1, 1, 2, 6], ExpectedValue::new_raw(1, 108)),
    ([1, 1, 3, 3], ExpectedValue::new_raw(1, 216)),
    ([1, 1, 3, 4], ExpectedValue::new_raw(1, 108)),
    ([1, 1, 3, 5], ExpectedValue::new_raw(1, 108)),
    ([1, 1, 3, 6], ExpectedValue::new_raw(1, 108)),
    ([1, 1, 4, 4], ExpectedValue::new_raw(1, 216)),
    ([1, 1, 4, 5], ExpectedValue::new_raw(1, 108)),
    ([1, 1, 4, 6], ExpectedValue::new_raw(1, 108)),
    ([1, 1, 5, 5], ExpectedValue::new_raw(1, 216)),
    ([1, 1, 5, 6], ExpectedValue::new_raw(1, 108)),
    ([1, 1, 6, 6], ExpectedValue::new_raw(1, 216)),
    ([1, 2, 2, 2], ExpectedValue::new_raw(1, 324)),
    ([1, 2, 2, 3], ExpectedValue::new_raw(1, 108)),
    ([1, 2, 2, 4], ExpectedValue::new_raw(1, 108)),
    ([1, 2, 2, 5], ExpectedValue::new_raw(1, 108)),
    ([1, 2, 2, 6], ExpectedValue::new_raw(1, 108)),
    ([1, 2, 3, 3], ExpectedValue::new_raw(1, 108)),
    ([1, 2, 3, 4], ExpectedValue::new_raw(1, 54)),
    ([1, 2, 3, 5], ExpectedValue::new_raw(1, 54)),
    ([1, 2, 3, 6], ExpectedValue::new_raw(1, 54)),
    ([1, 2, 4, 4], ExpectedValue::new_raw(1, 108)),
    ([1, 2, 4, 5], ExpectedValue::new_raw(1, 54)),
    ([1, 2, 4, 6], ExpectedValue::new_raw(1, 54)),
    ([1, 2, 5, 5], ExpectedValue::new_raw(1, 108)),
    ([1, 2, 5, 6], ExpectedValue::new_raw(1, 54)),
    ([1, 2, 6, 6], ExpectedValue::new_raw(1, 108)),
    ([1, 3, 3, 3], ExpectedValue::new_raw(1, 324)),
    ([1, 3, 3, 4], ExpectedValue::new_raw(1, 108)),
    ([1, 3, 3, 5], ExpectedValue::new_raw(1, 108)),
    ([1, 3, 3, 6], ExpectedValue::new_raw(1, 108)),
    ([1, 3, 4, 4], ExpectedValue::new_raw(1, 108)),
    ([1, 3, 4, 5], ExpectedValue::new_raw(1, 54)),
    ([1, 3, 4, 6], ExpectedValue::new_raw(1, 54)),
    ([1, 3, 5, 5], ExpectedValue::new_raw(1, 108)),
    ([1, 3, 5, 6], ExpectedValue::new_raw(1, 54)),
    ([1, 3, 6, 6], ExpectedValue::new_raw(1, 108)),
    ([1, 4, 4, 4], ExpectedValue::new_raw(1, 324)),
    ([1, 4, 4, 5], ExpectedValue::new_raw(1, 108)),
    ([1, 4, 4, 6], ExpectedValue::new_raw(1, 108)),
    ([1, 4, 5, 5], ExpectedValue::new_raw(1, 108)),
    ([1, 4, 5, 6], ExpectedValue::new_raw(1, 54)),
    ([1, 4, 6, 6], ExpectedValue::new_raw(1, 108)),
    ([1, 5, 5, 5], ExpectedValue::new_raw(1, 324)),
    ([1, 5, 5, 6], ExpectedValue::new_raw(1, 108)),
    ([1, 5, 6, 6], ExpectedValue::new_raw(1, 108)),
    ([1, 6, 6, 6], ExpectedValue::new_raw(1, 324)),
    ([2, 2, 2, 2], ExpectedValue::new_raw(1, 1296)),
    ([2, 2, 2, 3], ExpectedValue::new_raw(1, 324)),
    ([2, 2, 2, 4], ExpectedValue::new_raw(1, 324)),
    ([2, 2, 2, 5], ExpectedValue::new_raw(1, 324)),
    ([2, 2, 2, 6], ExpectedValue::new_raw(1, 324)),
    ([2, 2, 3, 3], ExpectedValue::new_raw(1, 216)),
    ([2, 2, 3, 4], ExpectedValue::new_raw(1, 108)),
    ([2, 2, 3, 5], ExpectedValue::new_raw(1, 108)),
    ([2, 2, 3, 6], ExpectedValue::new_raw(1, 108)),
    ([2, 2, 4, 4], ExpectedValue::new_raw(1, 216)),
    ([2, 2, 4, 5], ExpectedValue::new_raw(1, 108)),
    ([2, 2, 4, 6], ExpectedValue::new_raw(1, 108)),
    ([2, 2, 5, 5], ExpectedValue::new_raw(1, 216)),
    ([2, 2, 5, 6], ExpectedValue::new_raw(1, 108)),
    ([2, 2, 6, 6], ExpectedValue::new_raw(1, 216)),
    ([2, 3, 3, 3], ExpectedValue::new_raw(1, 324)),
    ([2, 3, 3, 4], ExpectedValue::new_raw(1, 108)),
    ([2, 3, 3, 5], ExpectedValue::new_raw(1, 108)),
    ([2, 3, 3, 6], ExpectedValue::new_raw(1, 108)),
    ([2, 3, 4, 4], ExpectedValue::new_raw(1, 108)),
    ([2, 3, 4, 5], ExpectedValue::new_raw(1, 54)),
    ([2, 3, 4, 6], ExpectedValue::new_raw(1, 54)),
    ([2, 3, 5, 5], ExpectedValue::new_raw(1, 108)),
    ([2, 3, 5, 6], ExpectedValue::new_raw(1, 54)),
    ([2, 3, 6, 6], ExpectedValue::new_raw(1, 108)),
    ([2, 4, 4, 4], ExpectedValue::new_raw(1, 324)),
    ([2, 4, 4, 5], ExpectedValue::new_raw(1, 108)),
    ([2, 4, 4, 6], ExpectedValue::new_raw(1, 108)),
    ([2, 4, 5, 5], ExpectedValue::new_raw(1, 108)),
    ([2, 4, 5, 6], ExpectedValue::new_raw(1, 54)),
    ([2, 4, 6, 6], ExpectedValue::new_raw(1, 108)),
    ([2, 5, 5, 5], ExpectedValue::new_raw(1, 324)),
    ([2, 5, 5, 6], ExpectedValue::new_raw(1, 108)),
    ([2, 5, 6, 6], ExpectedValue::new_raw(1, 108)),
    ([2, 6, 6, 6], ExpectedValue::new_raw(1, 324)),
    ([3, 3, 3, 3], ExpectedValue::new_raw(1, 1296)),
    ([3, 3, 3, 4], ExpectedValue::new_raw(1, 324)),
    ([3, 3, 3, 5], ExpectedValue::new_raw(1, 324)),
    ([3, 3, 3, 6], ExpectedValue::new_raw(1, 324)),
    ([3, 3, 4, 4], ExpectedValue::new_raw(1, 216)),
    ([3, 3, 4, 5], ExpectedValue::new_raw(1, 108)),
    ([3, 3, 4, 6], ExpectedValue::new_raw(1, 108)),
    ([3, 3, 5, 5], ExpectedValue::new_raw(1, 216)),
    ([3, 3, 5, 6], ExpectedValue::new_raw(1, 108)),
    ([3, 3, 6, 6], ExpectedValue::new_raw(1, 216)),
    ([3, 4, 4, 4], ExpectedValue::new_raw(1, 324)),
    ([3, 4, 4, 5], ExpectedValue::new_raw(1, 108)),
    ([3, 4, 4, 6], ExpectedValue::new_raw(1, 108)),
    ([3, 4, 5, 5], ExpectedValue::new_raw(1, 108)),
    ([3, 4, 5, 6], ExpectedValue::new_raw(1, 54)),
    ([3, 4, 6, 6], ExpectedValue::new_raw(1, 108)),
    ([3, 5, 5, 5], ExpectedValue::new_raw(1, 324)),
    ([3, 5, 5, 6], ExpectedValue::new_raw(1, 108)),
    ([3, 5, 6, 6], ExpectedValue::new_raw(1, 108)),
    ([3, 6, 6, 6], ExpectedValue::new_raw(1, 324)),
    ([4, 4, 4, 4], ExpectedValue::new_raw(1, 1296)),
    ([4, 4, 4, 5], ExpectedValue::new_raw(1, 324)),
    ([4, 4, 4, 6], ExpectedValue::new_raw(1, 324)),
    ([4, 4, 5, 5], ExpectedValue::new_raw(1, 216)),
    ([4, 4, 5, 6], ExpectedValue::new_raw(1, 108)),
    ([4, 4, 6, 6], ExpectedValue::new_raw(1, 216)),
    ([4, 5, 5, 5], ExpectedValue::new_raw(1, 324)),
    ([4, 5, 5, 6], ExpectedValue::new_raw(1, 108)),
    ([4, 5, 6, 6], ExpectedValue::new_raw(1, 108)),
    ([4, 6, 6, 6], ExpectedValue::new_raw(1, 324)),
    ([5, 5, 5, 5], ExpectedValue::new_raw(1, 1296)),
    ([5, 5, 5, 6], ExpectedValue::new_raw(1, 324)),
    ([5, 5, 6, 6], ExpectedValue::new_raw(1, 216)),
    ([5, 6, 6, 6], ExpectedValue::new_raw(1, 324)),
    ([6, 6, 6, 6], ExpectedValue::new_raw(1, 1296)),
];

const ROLL_5_PROB: [([Die; 5], ExpectedValue); 252] = [
    ([1, 1, 1, 1, 1], ExpectedValue::new_raw(1, 7776)),
    ([1, 1, 1, 1, 2], ExpectedValue::new_raw(5, 7776)),
    ([1, 1, 1, 1, 3], ExpectedValue::new_raw(5, 7776)),
    ([1, 1, 1, 1, 4], ExpectedValue::new_raw(5, 7776)),
    ([1, 1, 1, 1, 5], ExpectedValue::new_raw(5, 7776)),
    ([1, 1, 1, 1, 6], ExpectedValue::new_raw(5, 7776)),
    ([1, 1, 1, 2, 2], ExpectedValue::new_raw(5, 3888)),
    ([1, 1, 1, 2, 3], ExpectedValue::new_raw(5, 1944)),
    ([1, 1, 1, 2, 4], ExpectedValue::new_raw(5, 1944)),
    ([1, 1, 1, 2, 5], ExpectedValue::new_raw(5, 1944)),
    ([1, 1, 1, 2, 6], ExpectedValue::new_raw(5, 1944)),
    ([1, 1, 1, 3, 3], ExpectedValue::new_raw(5, 3888)),
    ([1, 1, 1, 3, 4], ExpectedValue::new_raw(5, 1944)),
    ([1, 1, 1, 3, 5], ExpectedValue::new_raw(5, 1944)),
    ([1, 1, 1, 3, 6], ExpectedValue::new_raw(5, 1944)),
    ([1, 1, 1, 4, 4], ExpectedValue::new_raw(5, 3888)),
    ([1, 1, 1, 4, 5], ExpectedValue::new_raw(5, 1944)),
    ([1, 1, 1, 4, 6], ExpectedValue::new_raw(5, 1944)),
    ([1, 1, 1, 5, 5], ExpectedValue::new_raw(5, 3888)),
    ([1, 1, 1, 5, 6], ExpectedValue::new_raw(5, 1944)),
    ([1, 1, 1, 6, 6], ExpectedValue::new_raw(5, 3888)),
    ([1, 1, 2, 2, 2], ExpectedValue::new_raw(5, 3888)),
    ([1, 1, 2, 2, 3], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 2, 2, 4], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 2, 2, 5], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 2, 2, 6], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 2, 3, 3], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 2, 3, 4], ExpectedValue::new_raw(5, 648)),
    ([1, 1, 2, 3, 5], ExpectedValue::new_raw(5, 648)),
    ([1, 1, 2, 3, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 1, 2, 4, 4], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 2, 4, 5], ExpectedValue::new_raw(5, 648)),
    ([1, 1, 2, 4, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 1, 2, 5, 5], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 2, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 1, 2, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 3, 3, 3], ExpectedValue::new_raw(5, 3888)),
    ([1, 1, 3, 3, 4], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 3, 3, 5], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 3, 3, 6], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 3, 4, 4], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 3, 4, 5], ExpectedValue::new_raw(5, 648)),
    ([1, 1, 3, 4, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 1, 3, 5, 5], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 3, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 1, 3, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 4, 4, 4], ExpectedValue::new_raw(5, 3888)),
    ([1, 1, 4, 4, 5], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 4, 4, 6], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 4, 5, 5], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 4, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 1, 4, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 5, 5, 5], ExpectedValue::new_raw(5, 3888)),
    ([1, 1, 5, 5, 6], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 5, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([1, 1, 6, 6, 6], ExpectedValue::new_raw(5, 3888)),
    ([1, 2, 2, 2, 2], ExpectedValue::new_raw(5, 7776)),
    ([1, 2, 2, 2, 3], ExpectedValue::new_raw(5, 1944)),
    ([1, 2, 2, 2, 4], ExpectedValue::new_raw(5, 1944)),
    ([1, 2, 2, 2, 5], ExpectedValue::new_raw(5, 1944)),
    ([1, 2, 2, 2, 6], ExpectedValue::new_raw(5, 1944)),
    ([1, 2, 2, 3, 3], ExpectedValue::new_raw(5, 1296)),
    ([1, 2, 2, 3, 4], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 2, 3, 5], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 2, 3, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 2, 4, 4], ExpectedValue::new_raw(5, 1296)),
    ([1, 2, 2, 4, 5], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 2, 4, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 2, 5, 5], ExpectedValue::new_raw(5, 1296)),
    ([1, 2, 2, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 2, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([1, 2, 3, 3, 3], ExpectedValue::new_raw(5, 1944)),
    ([1, 2, 3, 3, 4], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 3, 3, 5], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 3, 3, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 3, 4, 4], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 3, 4, 5], ExpectedValue::new_raw(5, 324)),
    ([1, 2, 3, 4, 6], ExpectedValue::new_raw(5, 324)),
    ([1, 2, 3, 5, 5], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 3, 5, 6], ExpectedValue::new_raw(5, 324)),
    ([1, 2, 3, 6, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 4, 4, 4], ExpectedValue::new_raw(5, 1944)),
    ([1, 2, 4, 4, 5], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 4, 4, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 4, 5, 5], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 4, 5, 6], ExpectedValue::new_raw(5, 324)),
    ([1, 2, 4, 6, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 5, 5, 5], ExpectedValue::new_raw(5, 1944)),
    ([1, 2, 5, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 5, 6, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 2, 6, 6, 6], ExpectedValue::new_raw(5, 1944)),
    ([1, 3, 3, 3, 3], ExpectedValue::new_raw(5, 7776)),
    ([1, 3, 3, 3, 4], ExpectedValue::new_raw(5, 1944)),
    ([1, 3, 3, 3, 5], ExpectedValue::new_raw(5, 1944)),
    ([1, 3, 3, 3, 6], ExpectedValue::new_raw(5, 1944)),
    ([1, 3, 3, 4, 4], ExpectedValue::new_raw(5, 1296)),
    ([1, 3, 3, 4, 5], ExpectedValue::new_raw(5, 648)),
    ([1, 3, 3, 4, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 3, 3, 5, 5], ExpectedValue::new_raw(5, 1296)),
    ([1, 3, 3, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 3, 3, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([1, 3, 4, 4, 4], ExpectedValue::new_raw(5, 1944)),
    ([1, 3, 4, 4, 5], ExpectedValue::new_raw(5, 648)),
    ([1, 3, 4, 4, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 3, 4, 5, 5], ExpectedValue::new_raw(5, 648)),
    ([1, 3, 4, 5, 6], ExpectedValue::new_raw(5, 324)),
    ([1, 3, 4, 6, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 3, 5, 5, 5], ExpectedValue::new_raw(5, 1944)),
    ([1, 3, 5, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 3, 5, 6, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 3, 6, 6, 6], ExpectedValue::new_raw(5, 1944)),
    ([1, 4, 4, 4, 4], ExpectedValue::new_raw(5, 7776)),
    ([1, 4, 4, 4, 5], ExpectedValue::new_raw(5, 1944)),
    ([1, 4, 4, 4, 6], ExpectedValue::new_raw(5, 1944)),
    ([1, 4, 4, 5, 5], ExpectedValue::new_raw(5, 1296)),
    ([1, 4, 4, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 4, 4, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([1, 4, 5, 5, 5], ExpectedValue::new_raw(5, 1944)),
    ([1, 4, 5, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 4, 5, 6, 6], ExpectedValue::new_raw(5, 648)),
    ([1, 4, 6, 6, 6], ExpectedValue::new_raw(5, 1944)),
    ([1, 5, 5, 5, 5], ExpectedValue::new_raw(5, 7776)),
    ([1, 5, 5, 5, 6], ExpectedValue::new_raw(5, 1944)),
    ([1, 5, 5, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([1, 5, 6, 6, 6], ExpectedValue::new_raw(5, 1944)),
    ([1, 6, 6, 6, 6], ExpectedValue::new_raw(5, 7776)),
    ([2, 2, 2, 2, 2], ExpectedValue::new_raw(1, 7776)),
    ([2, 2, 2, 2, 3], ExpectedValue::new_raw(5, 7776)),
    ([2, 2, 2, 2, 4], ExpectedValue::new_raw(5, 7776)),
    ([2, 2, 2, 2, 5], ExpectedValue::new_raw(5, 7776)),
    ([2, 2, 2, 2, 6], ExpectedValue::new_raw(5, 7776)),
    ([2, 2, 2, 3, 3], ExpectedValue::new_raw(5, 3888)),
    ([2, 2, 2, 3, 4], ExpectedValue::new_raw(5, 1944)),
    ([2, 2, 2, 3, 5], ExpectedValue::new_raw(5, 1944)),
    ([2, 2, 2, 3, 6], ExpectedValue::new_raw(5, 1944)),
    ([2, 2, 2, 4, 4], ExpectedValue::new_raw(5, 3888)),
    ([2, 2, 2, 4, 5], ExpectedValue::new_raw(5, 1944)),
    ([2, 2, 2, 4, 6], ExpectedValue::new_raw(5, 1944)),
    ([2, 2, 2, 5, 5], ExpectedValue::new_raw(5, 3888)),
    ([2, 2, 2, 5, 6], ExpectedValue::new_raw(5, 1944)),
    ([2, 2, 2, 6, 6], ExpectedValue::new_raw(5, 3888)),
    ([2, 2, 3, 3, 3], ExpectedValue::new_raw(5, 3888)),
    ([2, 2, 3, 3, 4], ExpectedValue::new_raw(5, 1296)),
    ([2, 2, 3, 3, 5], ExpectedValue::new_raw(5, 1296)),
    ([2, 2, 3, 3, 6], ExpectedValue::new_raw(5, 1296)),
    ([2, 2, 3, 4, 4], ExpectedValue::new_raw(5, 1296)),
    ([2, 2, 3, 4, 5], ExpectedValue::new_raw(5, 648)),
    ([2, 2, 3, 4, 6], ExpectedValue::new_raw(5, 648)),
    ([2, 2, 3, 5, 5], ExpectedValue::new_raw(5, 1296)),
    ([2, 2, 3, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([2, 2, 3, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([2, 2, 4, 4, 4], ExpectedValue::new_raw(5, 3888)),
    ([2, 2, 4, 4, 5], ExpectedValue::new_raw(5, 1296)),
    ([2, 2, 4, 4, 6], ExpectedValue::new_raw(5, 1296)),
    ([2, 2, 4, 5, 5], ExpectedValue::new_raw(5, 1296)),
    ([2, 2, 4, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([2, 2, 4, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([2, 2, 5, 5, 5], ExpectedValue::new_raw(5, 3888)),
    ([2, 2, 5, 5, 6], ExpectedValue::new_raw(5, 1296)),
    ([2, 2, 5, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([2, 2, 6, 6, 6], ExpectedValue::new_raw(5, 3888)),
    ([2, 3, 3, 3, 3], ExpectedValue::new_raw(5, 7776)),
    ([2, 3, 3, 3, 4], ExpectedValue::new_raw(5, 1944)),
    ([2, 3, 3, 3, 5], ExpectedValue::new_raw(5, 1944)),
    ([2, 3, 3, 3, 6], ExpectedValue::new_raw(5, 1944)),
    ([2, 3, 3, 4, 4], ExpectedValue::new_raw(5, 1296)),
    ([2, 3, 3, 4, 5], ExpectedValue::new_raw(5, 648)),
    ([2, 3, 3, 4, 6], ExpectedValue::new_raw(5, 648)),
    ([2, 3, 3, 5, 5], ExpectedValue::new_raw(5, 1296)),
    ([2, 3, 3, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([2, 3, 3, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([2, 3, 4, 4, 4], ExpectedValue::new_raw(5, 1944)),
    ([2, 3, 4, 4, 5], ExpectedValue::new_raw(5, 648)),
    ([2, 3, 4, 4, 6], ExpectedValue::new_raw(5, 648)),
    ([2, 3, 4, 5, 5], ExpectedValue::new_raw(5, 648)),
    ([2, 3, 4, 5, 6], ExpectedValue::new_raw(5, 324)),
    ([2, 3, 4, 6, 6], ExpectedValue::new_raw(5, 648)),
    ([2, 3, 5, 5, 5], ExpectedValue::new_raw(5, 1944)),
    ([2, 3, 5, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([2, 3, 5, 6, 6], ExpectedValue::new_raw(5, 648)),
    ([2, 3, 6, 6, 6], ExpectedValue::new_raw(5, 1944)),
    ([2, 4, 4, 4, 4], ExpectedValue::new_raw(5, 7776)),
    ([2, 4, 4, 4, 5], ExpectedValue::new_raw(5, 1944)),
    ([2, 4, 4, 4, 6], ExpectedValue::new_raw(5, 1944)),
    ([2, 4, 4, 5, 5], ExpectedValue::new_raw(5, 1296)),
    ([2, 4, 4, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([2, 4, 4, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([2, 4, 5, 5, 5], ExpectedValue::new_raw(5, 1944)),
    ([2, 4, 5, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([2, 4, 5, 6, 6], ExpectedValue::new_raw(5, 648)),
    ([2, 4, 6, 6, 6], ExpectedValue::new_raw(5, 1944)),
    ([2, 5, 5, 5, 5], ExpectedValue::new_raw(5, 7776)),
    ([2, 5, 5, 5, 6], ExpectedValue::new_raw(5, 1944)),
    ([2, 5, 5, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([2, 5, 6, 6, 6], ExpectedValue::new_raw(5, 1944)),
    ([2, 6, 6, 6, 6], ExpectedValue::new_raw(5, 7776)),
    ([3, 3, 3, 3, 3], ExpectedValue::new_raw(1, 7776)),
    ([3, 3, 3, 3, 4], ExpectedValue::new_raw(5, 7776)),
    ([3, 3, 3, 3, 5], ExpectedValue::new_raw(5, 7776)),
    ([3, 3, 3, 3, 6], ExpectedValue::new_raw(5, 7776)),
    ([3, 3, 3, 4, 4], ExpectedValue::new_raw(5, 3888)),
    ([3, 3, 3, 4, 5], ExpectedValue::new_raw(5, 1944)),
    ([3, 3, 3, 4, 6], ExpectedValue::new_raw(5, 1944)),
    ([3, 3, 3, 5, 5], ExpectedValue::new_raw(5, 3888)),
    ([3, 3, 3, 5, 6], ExpectedValue::new_raw(5, 1944)),
    ([3, 3, 3, 6, 6], ExpectedValue::new_raw(5, 3888)),
    ([3, 3, 4, 4, 4], ExpectedValue::new_raw(5, 3888)),
    ([3, 3, 4, 4, 5], ExpectedValue::new_raw(5, 1296)),
    ([3, 3, 4, 4, 6], ExpectedValue::new_raw(5, 1296)),
    ([3, 3, 4, 5, 5], ExpectedValue::new_raw(5, 1296)),
    ([3, 3, 4, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([3, 3, 4, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([3, 3, 5, 5, 5], ExpectedValue::new_raw(5, 3888)),
    ([3, 3, 5, 5, 6], ExpectedValue::new_raw(5, 1296)),
    ([3, 3, 5, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([3, 3, 6, 6, 6], ExpectedValue::new_raw(5, 3888)),
    ([3, 4, 4, 4, 4], ExpectedValue::new_raw(5, 7776)),
    ([3, 4, 4, 4, 5], ExpectedValue::new_raw(5, 1944)),
    ([3, 4, 4, 4, 6], ExpectedValue::new_raw(5, 1944)),
    ([3, 4, 4, 5, 5], ExpectedValue::new_raw(5, 1296)),
    ([3, 4, 4, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([3, 4, 4, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([3, 4, 5, 5, 5], ExpectedValue::new_raw(5, 1944)),
    ([3, 4, 5, 5, 6], ExpectedValue::new_raw(5, 648)),
    ([3, 4, 5, 6, 6], ExpectedValue::new_raw(5, 648)),
    ([3, 4, 6, 6, 6], ExpectedValue::new_raw(5, 1944)),
    ([3, 5, 5, 5, 5], ExpectedValue::new_raw(5, 7776)),
    ([3, 5, 5, 5, 6], ExpectedValue::new_raw(5, 1944)),
    ([3, 5, 5, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([3, 5, 6, 6, 6], ExpectedValue::new_raw(5, 1944)),
    ([3, 6, 6, 6, 6], ExpectedValue::new_raw(5, 7776)),
    ([4, 4, 4, 4, 4], ExpectedValue::new_raw(1, 7776)),
    ([4, 4, 4, 4, 5], ExpectedValue::new_raw(5, 7776)),
    ([4, 4, 4, 4, 6], ExpectedValue::new_raw(5, 7776)),
    ([4, 4, 4, 5, 5], ExpectedValue::new_raw(5, 3888)),
    ([4, 4, 4, 5, 6], ExpectedValue::new_raw(5, 1944)),
    ([4, 4, 4, 6, 6], ExpectedValue::new_raw(5, 3888)),
    ([4, 4, 5, 5, 5], ExpectedValue::new_raw(5, 3888)),
    ([4, 4, 5, 5, 6], ExpectedValue::new_raw(5, 1296)),
    ([4, 4, 5, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([4, 4, 6, 6, 6], ExpectedValue::new_raw(5, 3888)),
    ([4, 5, 5, 5, 5], ExpectedValue::new_raw(5, 7776)),
    ([4, 5, 5, 5, 6], ExpectedValue::new_raw(5, 1944)),
    ([4, 5, 5, 6, 6], ExpectedValue::new_raw(5, 1296)),
    ([4, 5, 6, 6, 6], ExpectedValue::new_raw(5, 1944)),
    ([4, 6, 6, 6, 6], ExpectedValue::new_raw(5, 7776)),
    ([5, 5, 5, 5, 5], ExpectedValue::new_raw(1, 7776)),
    ([5, 5, 5, 5, 6], ExpectedValue::new_raw(5, 7776)),
    ([5, 5, 5, 6, 6], ExpectedValue::new_raw(5, 3888)),
    ([5, 5, 6, 6, 6], ExpectedValue::new_raw(5, 3888)),
    ([5, 6, 6, 6, 6], ExpectedValue::new_raw(5, 7776)),
    ([6, 6, 6, 6, 6], ExpectedValue::new_raw(1, 7776)),
];
