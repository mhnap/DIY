use structural_convert::StructuralConvert;

fn main() {
    // Basic case
    #[derive(Debug, Clone, Copy, StructuralConvert)]
    #[convert(from(Int2))]
    struct Int1 {
        i: i32,
    }

    #[derive(Debug, Clone, Copy, StructuralConvert)]
    #[convert(from(Int1))]
    struct Int2 {
        i: i32,
    }

    let int1 = Int1 { i: 1 };
    dbg!(&int1);

    let int2: Int2 = int1.into();
    dbg!(&int2);

    let int1: Int1 = int2.into();
    dbg!(&int1);

    // Hierarchy case
    {
        #[derive(Debug, Clone, Copy, StructuralConvert)]
        #[convert(from(Range2))]
        struct Range1 {
            a: Int1,
            b: Int1,
        }

        #[derive(Debug, Clone, Copy, StructuralConvert)]
        #[convert(from(Range1))]
        struct Range2 {
            a: Int2,
            b: Int2,
        }

        let range1 = Range1 { a: int1, b: int1 };
        dbg!(&range1);

        let range2: Range2 = range1.into();
        dbg!(&range2);

        let range1: Range1 = range2.into();
        dbg!(&range1);
    }

    // Mixed case
    {
        #[derive(Debug, Clone, Copy, StructuralConvert)]
        #[convert(from(Range2))]
        struct Range1 {
            a: Int1,
            b: Int2,
        }

        #[derive(Debug, Clone, Copy, StructuralConvert)]
        #[convert(from(Range1))]
        struct Range2 {
            a: Int2,
            b: Int1,
        }

        let range1 = Range1 { a: int1, b: int2 };
        dbg!(&range1);

        let range2: Range2 = range1.into();
        dbg!(&range2);

        let range1: Range1 = range2.into();
        dbg!(&range1);
    }
}
