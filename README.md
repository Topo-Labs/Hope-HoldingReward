# Hope-HoldingReward

## Usage

1. 运行
```sh title="在项目根目录下"
make run
```

2. 停止运行
```sh title="在项目根目录下"
make stop
```

## List

1. api

```yaml
# 1. 获取当前奖励活动是第几期
GET /currentEpoch: { index: 1, startTime: 171020202020, endTime: 1712222222 }

# 2. 获取当前轮次的奖励榜单
GET rankList: [{address: 0x123, reward: 32500}, {..} ..]

# 3. 获取当前用户在当前轮次的待发放奖励
GET reward(address): { address: 0x123, reward: 31000 }

# 4. 获取所有历史奖励清单
GET historyRewardList: [{
	index: 1,
	startTime: 17100000000,
	endTime: 17200000000,
	list: [{address: 0x123, reward: 999}, {address: 0x234, reward: 888} ...]
}]
```