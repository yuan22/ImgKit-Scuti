## v1.2.3 更新日志

### 本次更新
- 修复 F2FS 回打包镜像的元数据写入问题, 纠正 NAT 双副本偏移与容量校验逻辑, 提升回提取稳定性。
- 修复 F2FS 提取配置生成, 补齐根目录 fs_config 记录及 file_contexts 根路径与 lost+found 规则输出。

<details>
<summary>English Version</summary>

## v1.2.3 Changelog

### Highlights
- Fixed F2FS repack metadata writing by correcting NAT dual-copy offset handling and copy-size validation, improving roundtrip extraction stability.
- Fixed F2FS config export by restoring root fs_config records and proper file_contexts rules for root and lost+found.

</details>
