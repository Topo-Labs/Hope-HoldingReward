# Design

> 本文尝试基于 **实时**，**流式**，**永续** 模式，计算每个推荐级别的 **持币奖励**。

## Database Schema Design

1. 推荐关系元数据

```rust title="视角是以关系为实体，而非下级或上级（这样理解不容易出错）"
ReferShip {
	lower: Address, // 某个推荐关系中的下级
	upper: Address, // 某个推荐关系中的上级
	timestamp: String, // 建立推荐关系的时间戳
}
```

2. 处于`推荐关系树`中的用户的交易Action (Buy, Sell, Transfer)

> 注: 这里刻意隐去了Buy/Sell/Transfer等动作的含义，只保留实质，以便统计 **有效持币量**。

> 注: 对于不在`推荐关系树`中的用户，不落库。

```rust
Action {
	address: Address,
	amount: i64, // 有符号整数（买，转入：为正；卖，转出：为负）
	timestamp: String, 
}
```

3. 已奖励

> 注：触发奖励的时机，详见 **奖励时机的触发**。应该分发奖励的，直接落库，直至官方财库（可以从数据库拉取）分发奖励。

> 注：`is_pay` 在初步落库时，为false；在实质性分发奖励之后，为true

> 注： `rewards` 中，分别为：直接推荐人，二级推荐人(可无)，三级推荐人(可无)

```rust
Reward {
	is_pay: bool,
	address: Address,
	startTime: String,
	endTime: String,
	amount: u64, // 有效持币量（并不一定等于某一次Buy/Transfer的量）（详见`有效持有量`的计算）
	rewards: [{address: Address, amount: u64}, {address: Address, amount: u64}, {address: Address, amount: u64}]
}
```

## Memory Schema Desgine

1. 推荐关系链

> 注: 系统启动时，从Database/ReferShip中提取，解析，并注入。

```rust
type Lower = Address;
type Upper = Address;

ReferMap: HashMap[Lower]{ Upper, Timestamp }
```

所提供的方法：

- referTree(address): 给定某个地址，返回从该地址往下的推荐树
- referChain(address): 给定某个地址，返回从该地址往上的推荐链（注：最终结构要凑够 `Some[Address, None, None]` ）

2. 操作Action流

> 注: ActionFlow中过的Action_N 并不严格等于 Database 中的 Action。Action_N 是由 Action 计算而来（基于 **有效持币数** 的算法）。因此在系统启动时，需要从数据库中读取，并逐个经过**计算**后，再注入内存

```rust
ActionFlow: HashMap[Address]{Action1, Action2, ...}
```

3. 奖励统计树

> 注：RewardTree 是由 ReferMap 和 ActionFlow 计算而来。并随着两者的更新而更新。由ReferMap来完善整个RewardTree（可能有N级）；由ActionFlow来 **往上** 更新三级。

> 注：当ActionFlow更新（卖，转出）时，减少该Action的时间戳对应的amount（可能往前追溯，即可能减少的不止一个amount）；当ActionFlow更新（买，转入）时，新增 `RewardItem{address, amount, timestamp}`

```rust
type A = Address;
type B = Address;
type C = Address;

RewardTree: {
	A: {
		rewards: [{amount: u64, timestamp: String, address: Address}, ..],
		B: {
			rewards: [{amount: u64, timestamp: String, address: Address}, ..],
			C: {
				rewards: [{amount: u64, timestamp: String, address: Address}, ..],
				D: // 可能持续下去（即不止三级奖励树）
			},
			..
		},
		..
	},
	..
}
```

提供的方法：
- reward(address): 获取RewardTree中某个地址的待分发奖励清单

> 注!!!: 这里在构建 RewardTree 时，可能不止

