# Terraform Sign Up

## Dependencies

- [Terraform >= 1.0.10][6]
- [AWS Provider == 3.63.0][7]

## Init

One shot

```
AWS_PROFILE=profile_name terraform init
```

## Plan

Example create development infra. You can replace the dev.tfvars file with
prod.tfvars

```
AWS_PROFILE=profile_name terraform plan -var-file="env/dev.tfvars"
```

## Apply

```
AWS_PROFILE=profile_name terraform apply -var-file="env/dev.tfvars"
```

## Destroy (Danger)

```
AWS_PROFILE=profile_name terraform destroy -var-file="env/dev.tfvars"
```

## Output

It is common to have to get the value of a variable from some resource for this
we use the output command.

```
AWS_PROFILE=profile_name terraform output docker_uri
```

Output list:

- docker\_uri: uri image (ECR)
- repo\_ssh: ssh url git repo (Code Commit)
- repo\_http: http url git repo (Code Commit)

[6]: https://www.terraform.io/
[7]: https://registry.terraform.io/providers/hashicorp/aws/latest/docs
