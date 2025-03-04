# api

1. api

```yaml
# 2. 获取当前轮次的奖励榜单
GET rankList: [{address: 0x123, reward: 32500}, {..} ..]

# 3. 获取当前用户在当前轮次的待发放奖励
#    # level: -1,-2,-3 (分别为当前层级往下的第一级，第二级，第三极)
GET reward(address): [{ level: -1, address: 0x123, reward: 31000, timestamp: 17100001234 }, ..]

# 4. 获取所有历史奖励清单
GET historyRewardList: 
[{
	address: 0x123,
	reward: 999000000
}, 
..]
```