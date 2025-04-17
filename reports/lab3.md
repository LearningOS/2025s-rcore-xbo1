## 编程作业
sys_spawn 通过现有的fork和exec实现，注意trap_cx的返回值即可。

stride 调度算法实现需要先在TaskControlBlockInner中保存stride和priority，在TaskManager的add方法中设置task的新stride值，当超过BIG_STRIDE时，更新为BIG_STRIDE，并添加到列表最后。在fetch方法中，遍历所有task，找到第一个最小stride的task，并删除去除，这样可以保证stride在BIG_STRIDE时也能轮流调度到每个task。

## 简答作业
1. 实际情况是轮到 p1 执行吗？为什么？
> 实际情况时轮到p2执行，因为(250+10)=260>255大于无符号8bit能存储的最大值，溢出后会变成4，4<255，按照stride算法，还是轮到p2执行。

2. 为什么？尝试简单说明
> 当priority>=2 时，每个task的pass=BigStride/priority,MAX_PASS必定<=BigStride/2, 而任何时候，STRIDE_MIN + MAX_PASS >= STRIDE_MAX 总是成立的。所以STRIDE_MAX - STRIDE_MIN <= MAX_PASS <= BigStride/2

3. 已知以上结论，考虑溢出的情况下，可以为 Stride 设计特别的比较器，让 BinaryHeap<Stride> 的 pop 方法能返回真正最小的 Stride。补全下列代码中的 partial_cmp 函数，假设两个 Stride 永远不会相等。
```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // big_stide 是全局定义的BigStride
        let big_stide = 0x7ffffff;
        let ret = self.0 - other.0;
        if ret > big_stide/2 {
            return Some(Ordering::Less)
        }
        else {
            return Some(Ordering::Greater)
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```
  



## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> 有通过搜索引擎查询知识点，参考文档：https://rcore-os.cn/rCore-Tutorial-Book-v3/

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。