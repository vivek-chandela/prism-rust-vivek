# Prism Distributed Testbed

## Setting Up

1. Install jq
2. Install AWS CLI tool and configure the IAM Key and Region.
3. Place the SSH key at `~/.ssh/prism.pem`
4. Place this line `Include config.d/prism` at the beginning of `~/.ssh/config`
5. Execute `mkdir -p ~/.ssh/config.d`

## Usage

Run `./run.sh help` to view a list of available commands.

### Log Files

instances.txt records the EC2 instances that are started in the following
format:

```
<Instance ID>,<Public IP>,<VPC IP>
```

nodes.txt records the Scorex nodes that are started, in the following format:

```
<Node Name>,<EC2 ID>,<Public IP>,<VPC IP>,<API IP>,<P2P IP>
```
