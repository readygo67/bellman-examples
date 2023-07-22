# bellman-examples


使用bellman的核心就是要实现 Circuit<Scalar: PrimeField> 这个trait。bellman中的变量(Variable)包括电路的instance和witness，按照
先赋值，再alloc变量，最后施加约束的步骤进行。例如在cube.rs中 

        // Allocate: x * x = s1， s1_val赋值
        let s1_val = x_val.map(|e| {
            e.square()
        });
        // alloc s1 变量，s1的值是s1_val
        let s1 = cs.alloc(|| "s1", || {
            s1_val.ok_or(SynthesisError::AssignmentMissing)
        })?;     
        // Enforce: x * x = s1, 按照r1cs的规则 施加约束 x * x = s1
        cs.enforce(
            || "x * x = s1",
            |lc: lc + x,
            |lc| lc + x,
            |lc| lc + s1
        );

## Reference
https://electriccoin.co/blog/bellman-zksnarks-in-rust/ 这篇文章虽然有些过时了，但是基本讲清楚了bellman的流程。

https://trapdoortech.medium.com/zkp-deep-into-bellman-library-9b1bf52cb1a6 讲解了bellman内部原理。

