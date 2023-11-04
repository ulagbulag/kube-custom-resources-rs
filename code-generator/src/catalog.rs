// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

pub const CRD_V1_SOURCES: &'static [UpstreamSource] = &[
    UpstreamSource {
        project_name: "3scale/apicast-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/3scale/apicast-operator/blob/master/config/crd/bases/apps.3scale.net_apicasts.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aerospike/aerospike-kubernetes-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aerospike/aerospike-kubernetes-operator/blob/master/config/crd/bases/asdb.aerospike.com_aerospikeclusters.yaml",
        ],
    },
    UpstreamSource {
        project_name: "Alvearie/imaging-ingestion",
        license: APACHE_V2,
        urls: &[
            "https://github.com/Alvearie/imaging-ingestion/blob/main/imaging-ingestion-operator/config/crd/bases/imaging-ingestion.alvearie.org_dicomeventbridges.yaml",
            "https://github.com/Alvearie/imaging-ingestion/blob/main/imaging-ingestion-operator/config/crd/bases/imaging-ingestion.alvearie.org_dicomeventdriveningestions.yaml",
            "https://github.com/Alvearie/imaging-ingestion/blob/main/imaging-ingestion-operator/config/crd/bases/imaging-ingestion.alvearie.org_dicominstancebindings.yaml",
            "https://github.com/Alvearie/imaging-ingestion/blob/main/imaging-ingestion-operator/config/crd/bases/imaging-ingestion.alvearie.org_dicomstudybindings.yaml",
            "https://github.com/Alvearie/imaging-ingestion/blob/main/imaging-ingestion-operator/config/crd/bases/imaging-ingestion.alvearie.org_dicomwebingestionservices.yaml",
            "https://github.com/Alvearie/imaging-ingestion/blob/main/imaging-ingestion-operator/config/crd/bases/imaging-ingestion.alvearie.org_dimseingestionservices.yaml",
            "https://github.com/Alvearie/imaging-ingestion/blob/main/imaging-ingestion-operator/config/crd/bases/imaging-ingestion.alvearie.org_dimseproxies.yaml",
        ],
    },
    UpstreamSource {
        project_name: "ansible/awx-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/ansible/awx-operator/blob/devel/config/crd/bases/awx.ansible.com_awxbackups.yaml",
            "https://github.com/ansible/awx-operator/blob/devel/config/crd/bases/awx.ansible.com_awxrestores.yaml",
            "https://github.com/ansible/awx-operator/blob/devel/config/crd/bases/awx.ansible.com_awxs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "apache/camel-k",
        license: APACHE_V2,
        urls: &[
            "https://github.com/apache/camel-k/blob/main/config/crd/bases/camel.apache.org_builds.yaml",
            "https://github.com/apache/camel-k/blob/main/config/crd/bases/camel.apache.org_camelcatalogs.yaml",
            "https://github.com/apache/camel-k/blob/main/config/crd/bases/camel.apache.org_integrationkits.yaml",
            "https://github.com/apache/camel-k/blob/main/config/crd/bases/camel.apache.org_integrationplatforms.yaml",
            "https://github.com/apache/camel-k/blob/main/config/crd/bases/camel.apache.org_integrations.yaml",
            "https://github.com/apache/camel-k/blob/main/config/crd/bases/camel.apache.org_kameletbindings.yaml",
            "https://github.com/apache/camel-k/blob/main/config/crd/bases/camel.apache.org_kamelets.yaml",
        ],
    },
    UpstreamSource {
        project_name: "apache/flink-kubernetes-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/apache/flink-kubernetes-operator/blob/main/helm/flink-kubernetes-operator/crds/flinkdeployments.flink.apache.org-v1.yml",
            "https://github.com/apache/flink-kubernetes-operator/blob/main/helm/flink-kubernetes-operator/crds/flinksessionjobs.flink.apache.org-v1.yml",
        ],
    },
    UpstreamSource {
        project_name: "apache/rocketmq-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/apache/rocketmq-operator/blob/master/deploy/crds/rocketmq.apache.org_brokers.yaml",
            "https://github.com/apache/rocketmq-operator/blob/master/deploy/crds/rocketmq.apache.org_consoles.yaml",
            "https://github.com/apache/rocketmq-operator/blob/master/deploy/crds/rocketmq.apache.org_nameservices.yaml",
            "https://github.com/apache/rocketmq-operator/blob/master/deploy/crds/rocketmq.apache.org_topictransfers.yaml",
        ],
    },
    UpstreamSource {
        project_name: "Apicurio/apicurio-registry-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/Apicurio/apicurio-registry-operator/blob/main/config/crd/resources/registry.apicur.io_apicurioregistries.yaml",
        ],
    },
    UpstreamSource {
        project_name: "apimatic/apimatic-kubernetes-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/apimatic/apimatic-kubernetes-operator/blob/main/config/crd/bases/apicodegen.apimatic.io_apimatics.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aquasecurity/aqua-operator",
        license: AQUA,
        urls: &[
            "https://github.com/aquasecurity/aqua-operator/blob/master/config/crd/bases/aquasecurity.github.io_aquastarboards.yaml",
            "https://github.com/aquasecurity/aqua-operator/blob/master/config/crd/bases/aquasecurity.github.io_clusterconfigauditreports.yaml",
            "https://github.com/aquasecurity/aqua-operator/blob/master/config/crd/bases/aquasecurity.github.io_configauditreports.yaml",
            "https://github.com/aquasecurity/aqua-operator/blob/master/config/crd/bases/operator.aquasec.com_aquacsps.yaml",
            "https://github.com/aquasecurity/aqua-operator/blob/master/config/crd/bases/operator.aquasec.com_aquadatabases.yaml",
            "https://github.com/aquasecurity/aqua-operator/blob/master/config/crd/bases/operator.aquasec.com_aquaenforcers.yaml",
            "https://github.com/aquasecurity/aqua-operator/blob/master/config/crd/bases/operator.aquasec.com_aquagateways.yaml",
            "https://github.com/aquasecurity/aqua-operator/blob/master/config/crd/bases/operator.aquasec.com_aquakubeenforcers.yaml",
            "https://github.com/aquasecurity/aqua-operator/blob/master/config/crd/bases/operator.aquasec.com_aquascanners.yaml",
            "https://github.com/aquasecurity/aqua-operator/blob/master/config/crd/bases/operator.aquasec.com_aquaservers.yaml",
        ],
    },
    UpstreamSource {
        project_name: "argoproj-labs/argocd-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/argoproj-labs/argocd-operator/blob/master/config/crd/bases/argoproj.io_applications.yaml",
            "https://github.com/argoproj-labs/argocd-operator/blob/master/config/crd/bases/argoproj.io_applicationsets.yaml",
            "https://github.com/argoproj-labs/argocd-operator/blob/master/config/crd/bases/argoproj.io_appprojects.yaml",
            "https://github.com/argoproj-labs/argocd-operator/blob/master/config/crd/bases/argoproj.io_argocdexports.yaml",
            "https://github.com/argoproj-labs/argocd-operator/blob/master/config/crd/bases/argoproj.io_argocds.yaml",
        ],
    },
    UpstreamSource {
        project_name: "atlasmap/atlasmap-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/atlasmap/atlasmap-operator/blob/main/config/crd/bases/atlasmap.io_atlasmaps.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/apigatewayv2-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/apigatewayv2-controller/blob/main/config/crd/bases/apigatewayv2.services.k8s.aws_apis.yaml",
            "https://github.com/aws-controllers-k8s/apigatewayv2-controller/blob/main/config/crd/bases/apigatewayv2.services.k8s.aws_authorizers.yaml",
            "https://github.com/aws-controllers-k8s/apigatewayv2-controller/blob/main/config/crd/bases/apigatewayv2.services.k8s.aws_deployments.yaml",
            "https://github.com/aws-controllers-k8s/apigatewayv2-controller/blob/main/config/crd/bases/apigatewayv2.services.k8s.aws_integrations.yaml",
            "https://github.com/aws-controllers-k8s/apigatewayv2-controller/blob/main/config/crd/bases/apigatewayv2.services.k8s.aws_routes.yaml",
            "https://github.com/aws-controllers-k8s/apigatewayv2-controller/blob/main/config/crd/bases/apigatewayv2.services.k8s.aws_stages.yaml",
            "https://github.com/aws-controllers-k8s/apigatewayv2-controller/blob/main/config/crd/bases/apigatewayv2.services.k8s.aws_vpclinks.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/applicationautoscaling-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/applicationautoscaling-controller/blob/main/config/crd/bases/applicationautoscaling.services.k8s.aws_scalabletargets.yaml",
            "https://github.com/aws-controllers-k8s/applicationautoscaling-controller/blob/main/config/crd/bases/applicationautoscaling.services.k8s.aws_scalingpolicies.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/dynamodb-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/dynamodb-controller/blob/main/config/crd/bases/dynamodb.services.k8s.aws_backups.yaml",
            "https://github.com/aws-controllers-k8s/dynamodb-controller/blob/main/config/crd/bases/dynamodb.services.k8s.aws_globaltables.yaml",
            "https://github.com/aws-controllers-k8s/dynamodb-controller/blob/main/config/crd/bases/dynamodb.services.k8s.aws_tables.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/ec2-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/ec2-controller/blob/main/config/crd/bases/ec2.services.k8s.aws_dhcpoptions.yaml",
            "https://github.com/aws-controllers-k8s/ec2-controller/blob/main/config/crd/bases/ec2.services.k8s.aws_elasticipaddresses.yaml",
            "https://github.com/aws-controllers-k8s/ec2-controller/blob/main/config/crd/bases/ec2.services.k8s.aws_instances.yaml",
            "https://github.com/aws-controllers-k8s/ec2-controller/blob/main/config/crd/bases/ec2.services.k8s.aws_internetgateways.yaml",
            "https://github.com/aws-controllers-k8s/ec2-controller/blob/main/config/crd/bases/ec2.services.k8s.aws_natgateways.yaml",
            "https://github.com/aws-controllers-k8s/ec2-controller/blob/main/config/crd/bases/ec2.services.k8s.aws_routetables.yaml",
            "https://github.com/aws-controllers-k8s/ec2-controller/blob/main/config/crd/bases/ec2.services.k8s.aws_securitygroups.yaml",
            "https://github.com/aws-controllers-k8s/ec2-controller/blob/main/config/crd/bases/ec2.services.k8s.aws_subnets.yaml",
            "https://github.com/aws-controllers-k8s/ec2-controller/blob/main/config/crd/bases/ec2.services.k8s.aws_transitgateways.yaml",
            "https://github.com/aws-controllers-k8s/ec2-controller/blob/main/config/crd/bases/ec2.services.k8s.aws_vpcendpoints.yaml",
            "https://github.com/aws-controllers-k8s/ec2-controller/blob/main/config/crd/bases/ec2.services.k8s.aws_vpcs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/ecr-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/ecr-controller/blob/main/config/crd/bases/ecr.services.k8s.aws_pullthroughcacherules.yaml",
            "https://github.com/aws-controllers-k8s/ecr-controller/blob/main/config/crd/bases/ecr.services.k8s.aws_repositories.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/eks-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/eks-controller/blob/main/config/crd/bases/eks.services.k8s.aws_addons.yaml",
            "https://github.com/aws-controllers-k8s/eks-controller/blob/main/config/crd/bases/eks.services.k8s.aws_clusters.yaml",
            "https://github.com/aws-controllers-k8s/eks-controller/blob/main/config/crd/bases/eks.services.k8s.aws_fargateprofiles.yaml",
            "https://github.com/aws-controllers-k8s/eks-controller/blob/main/config/crd/bases/eks.services.k8s.aws_nodegroups.yaml",
            "https://github.com/aws-controllers-k8s/eks-controller/blob/main/config/crd/common/bases/services.k8s.aws_adoptedresources.yaml",
            "https://github.com/aws-controllers-k8s/eks-controller/blob/main/config/crd/common/bases/services.k8s.aws_fieldexports.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/elasticache-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/elasticache-controller/blob/main/config/crd/bases/elasticache.services.k8s.aws_cacheparametergroups.yaml",
            "https://github.com/aws-controllers-k8s/elasticache-controller/blob/main/config/crd/bases/elasticache.services.k8s.aws_cachesubnetgroups.yaml",
            "https://github.com/aws-controllers-k8s/elasticache-controller/blob/main/config/crd/bases/elasticache.services.k8s.aws_replicationgroups.yaml",
            "https://github.com/aws-controllers-k8s/elasticache-controller/blob/main/config/crd/bases/elasticache.services.k8s.aws_snapshots.yaml",
            "https://github.com/aws-controllers-k8s/elasticache-controller/blob/main/config/crd/bases/elasticache.services.k8s.aws_usergroups.yaml",
            "https://github.com/aws-controllers-k8s/elasticache-controller/blob/main/config/crd/bases/elasticache.services.k8s.aws_users.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/emrcontainers-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/emrcontainers-controller/blob/main/config/crd/bases/emrcontainers.services.k8s.aws_jobruns.yaml",
            "https://github.com/aws-controllers-k8s/emrcontainers-controller/blob/main/config/crd/bases/emrcontainers.services.k8s.aws_virtualclusters.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/iam-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/iam-controller/blob/main/config/crd/bases/iam.services.k8s.aws_groups.yaml",
            "https://github.com/aws-controllers-k8s/iam-controller/blob/main/config/crd/bases/iam.services.k8s.aws_policies.yaml",
            "https://github.com/aws-controllers-k8s/iam-controller/blob/main/config/crd/bases/iam.services.k8s.aws_roles.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/kms-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/kms-controller/blob/main/config/crd/bases/kms.services.k8s.aws_aliases.yaml",
            "https://github.com/aws-controllers-k8s/kms-controller/blob/main/config/crd/bases/kms.services.k8s.aws_grants.yaml",
            "https://github.com/aws-controllers-k8s/kms-controller/blob/main/config/crd/bases/kms.services.k8s.aws_keys.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/lambda-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/lambda-controller/blob/main/config/crd/bases/lambda.services.k8s.aws_aliases.yaml",
            "https://github.com/aws-controllers-k8s/lambda-controller/blob/main/config/crd/bases/lambda.services.k8s.aws_codesigningconfigs.yaml",
            "https://github.com/aws-controllers-k8s/lambda-controller/blob/main/config/crd/bases/lambda.services.k8s.aws_eventsourcemappings.yaml",
            "https://github.com/aws-controllers-k8s/lambda-controller/blob/main/config/crd/bases/lambda.services.k8s.aws_functions.yaml",
            "https://github.com/aws-controllers-k8s/lambda-controller/blob/main/config/crd/bases/lambda.services.k8s.aws_functionurlconfigs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/mq-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/mq-controller/blob/main/config/crd/bases/mq.services.k8s.aws_brokers.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/opensearchservice-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/opensearchservice-controller/blob/main/config/crd/bases/opensearchservice.services.k8s.aws_domains.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/prometheusservice-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/prometheusservice-controller/blob/main/config/crd/bases/prometheusservice.services.k8s.aws_alertmanagerdefinitions.yaml",
            "https://github.com/aws-controllers-k8s/prometheusservice-controller/blob/main/config/crd/bases/prometheusservice.services.k8s.aws_rulegroupsnamespaces.yaml",
            "https://github.com/aws-controllers-k8s/prometheusservice-controller/blob/main/config/crd/bases/prometheusservice.services.k8s.aws_workspaces.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/rds-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/rds-controller/blob/main/config/crd/bases/rds.services.k8s.aws_dbclusterparametergroups.yaml",
            "https://github.com/aws-controllers-k8s/rds-controller/blob/main/config/crd/bases/rds.services.k8s.aws_dbclusters.yaml",
            "https://github.com/aws-controllers-k8s/rds-controller/blob/main/config/crd/bases/rds.services.k8s.aws_dbinstances.yaml",
            "https://github.com/aws-controllers-k8s/rds-controller/blob/main/config/crd/bases/rds.services.k8s.aws_dbparametergroups.yaml",
            "https://github.com/aws-controllers-k8s/rds-controller/blob/main/config/crd/bases/rds.services.k8s.aws_dbproxies.yaml",
            "https://github.com/aws-controllers-k8s/rds-controller/blob/main/config/crd/bases/rds.services.k8s.aws_dbsubnetgroups.yaml",
            "https://github.com/aws-controllers-k8s/rds-controller/blob/main/config/crd/bases/rds.services.k8s.aws_globalclusters.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/s3-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/s3-controller/blob/main/config/crd/bases/s3.services.k8s.aws_buckets.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/sagemaker-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_apps.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_dataqualityjobdefinitions.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_domains.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_endpointconfigs.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_endpoints.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_featuregroups.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_hyperparametertuningjobs.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_modelbiasjobdefinitions.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_modelexplainabilityjobdefinitions.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_modelpackagegroups.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_modelpackages.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_modelqualityjobdefinitions.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_models.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_monitoringschedules.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_notebookinstancelifecycleconfigs.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_notebookinstances.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_processingjobs.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_trainingjobs.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_transformjobs.yaml",
            "https://github.com/aws-controllers-k8s/sagemaker-controller/blob/main/config/crd/bases/sagemaker.services.k8s.aws_userprofiles.yaml",
        ],
    },
    UpstreamSource {
        project_name: "aws-controllers-k8s/sfn-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/aws-controllers-k8s/sfn-controller/blob/main/config/crd/bases/sfn.services.k8s.aws_activities.yaml",
            "https://github.com/aws-controllers-k8s/sfn-controller/blob/main/config/crd/bases/sfn.services.k8s.aws_statemachines.yaml",
        ],
    },
    UpstreamSource {
        project_name: "b3scale/b3scale-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/b3scale/b3scale-operator/blob/main/kubernetes/crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "bitnami-labs/sealed-secrets",
        license: APACHE_V2,
        urls: &[
            "https://github.com/bitnami-labs/sealed-secrets/blob/main/helm/sealed-secrets/crds/bitnami.com_sealedsecrets.yaml",
        ],
    },
    UpstreamSource {
        project_name: "carlosedp/lbconfig-operator",
        license: MIT,
        urls: &[
            "https://github.com/carlosedp/lbconfig-operator/blob/main/config/crd/bases/lb.lbconfig.carlosedp.com_externalloadbalancers.yaml",
        ],
    },
    UpstreamSource {
        project_name: "cert-manager/cert-manager",
        license: APACHE_V2,
        urls: &[
            "https://github.com/cert-manager/cert-manager/blob/master/deploy/crds/crd-certificaterequests.yaml",
            "https://github.com/cert-manager/cert-manager/blob/master/deploy/crds/crd-certificates.yaml",
            "https://github.com/cert-manager/cert-manager/blob/master/deploy/crds/crd-challenges.yaml",
            "https://github.com/cert-manager/cert-manager/blob/master/deploy/crds/crd-clusterissuers.yaml",
            "https://github.com/cert-manager/cert-manager/blob/master/deploy/crds/crd-issuers.yaml",
            "https://github.com/cert-manager/cert-manager/blob/master/deploy/crds/crd-orders.yaml",
        ],
    },
    UpstreamSource {
        project_name: "chaos-mesh/chaos-mesh",
        license: APACHE_V2,
        urls: &[
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_awschaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_azurechaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_blockchaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_dnschaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_gcpchaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_httpchaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_iochaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_jvmchaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_kernelchaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_networkchaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_physicalmachinechaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_physicalmachines.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_podchaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_podhttpchaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_podiochaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_podnetworkchaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_remoteclusters.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_schedules.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_statuschecks.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_stresschaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_timechaos.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_workflownodes.yaml",
            "https://github.com/chaos-mesh/chaos-mesh/blob/master/config/crd/bases/chaos-mesh.org_workflows.yaml",
        ],
    },
    UpstreamSource {
        project_name: "che-incubator/kubernetes-image-puller-operator",
        license: EPL_V2,
        urls: &[
            "https://github.com/che-incubator/kubernetes-image-puller-operator/blob/main/config/crd/bases/che.eclipse.org_kubernetesimagepullers.yaml",
        ],
    },
    UpstreamSource {
        project_name: "cilium/cilium",
        license: APACHE_V2,
        urls: &[
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2/ciliumclusterwideenvoyconfigs.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2/ciliumclusterwidenetworkpolicies.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2/ciliumegressgatewaypolicies.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2/ciliumendpoints.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2/ciliumenvoyconfigs.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2/ciliumexternalworkloads.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2/ciliumidentities.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2/ciliumlocalredirectpolicies.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2/ciliumnetworkpolicies.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2/ciliumnodes.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2alpha1/ciliumbgppeeringpolicies.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2alpha1/ciliumcidrgroups.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2alpha1/ciliumendpointslices.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2alpha1/ciliuml2announcementpolicies.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2alpha1/ciliumloadbalancerippools.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2alpha1/ciliumnodeconfigs.yaml",
            "https://github.com/cilium/cilium/blob/main/pkg/k8s/apis/cilium.io/client/crds/v2alpha1/ciliumpodippools.yaml",
        ],
    },
    UpstreamSource {
        project_name: "clastix/capsule",
        license: APACHE_V2,
        urls: &[
            "https://github.com/clastix/capsule/blob/master/config/crd/bases/capsule.clastix.io_capsuleconfigurations.yaml",
            "https://github.com/clastix/capsule/blob/master/config/crd/bases/capsule.clastix.io_tenants.yaml",
        ],
    },
    UpstreamSource {
        project_name: "cloud-bulldozer/benchmark-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/cloud-bulldozer/benchmark-operator/blob/master/config/crd/bases/ripsaw.cloudbulldozer.io_benchmarks.yaml",
        ],
    },
    UpstreamSource {
        project_name: "clusternet/clusternet",
        license: APACHE_V2,
        urls: &[
            "https://github.com/clusternet/clusternet/blob/main/manifests/crds/apps.clusternet.io_bases.yaml",
            "https://github.com/clusternet/clusternet/blob/main/manifests/crds/apps.clusternet.io_descriptions.yaml",
            "https://github.com/clusternet/clusternet/blob/main/manifests/crds/apps.clusternet.io_feedinventories.yaml",
            "https://github.com/clusternet/clusternet/blob/main/manifests/crds/apps.clusternet.io_globalizations.yaml",
            "https://github.com/clusternet/clusternet/blob/main/manifests/crds/apps.clusternet.io_helmcharts.yaml",
            "https://github.com/clusternet/clusternet/blob/main/manifests/crds/apps.clusternet.io_helmreleases.yaml",
            "https://github.com/clusternet/clusternet/blob/main/manifests/crds/apps.clusternet.io_localizations.yaml",
            "https://github.com/clusternet/clusternet/blob/main/manifests/crds/apps.clusternet.io_manifests.yaml",
            "https://github.com/clusternet/clusternet/blob/main/manifests/crds/apps.clusternet.io_subscriptions.yaml",
            "https://github.com/clusternet/clusternet/blob/main/manifests/crds/clusters.clusternet.io_clusterregistrationrequests.yaml",
            "https://github.com/clusternet/clusternet/blob/main/manifests/crds/clusters.clusternet.io_managedclusters.yaml",
            "https://github.com/clusternet/clusternet/blob/main/manifests/crds/nodefeaturerule-crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "clusterpedia-io/clusterpedia",
        license: APACHE_V2,
        urls: &[
            "https://github.com/clusterpedia-io/clusterpedia/blob/main/kustomize/crds/cluster.clusterpedia.io_clustersyncresources.yaml",
            "https://github.com/clusterpedia-io/clusterpedia/blob/main/kustomize/crds/cluster.clusterpedia.io_pediaclusters.yaml",
            "https://github.com/clusterpedia-io/clusterpedia/blob/main/kustomize/crds/policy.clusterpedia.io_clusterimportpolicies.yaml",
            "https://github.com/clusterpedia-io/clusterpedia/blob/main/kustomize/crds/policy.clusterpedia.io_pediaclusterlifecycles.yaml",
        ],
    },
    UpstreamSource {
        project_name: "composable-operator/composable",
        license: APACHE_V2,
        urls: &[
            "https://github.com/composable-operator/composable/blob/main/config/crd/bases/ibmcloud.ibm.com_composables.yaml",
        ],
    },
    UpstreamSource {
        project_name: "couchbase-partners/helm-charts",
        license: APACHE_V2,
        urls: &[
            "https://github.com/couchbase-partners/helm-charts/blob/master/charts/couchbase-operator/crds/couchbase.crds.yaml",
        ],
    },
    UpstreamSource {
        project_name: "crossplane/crossplane",
        license: APACHE_V2,
        urls: &[
            "https://github.com/crossplane/crossplane/blob/master/cluster/crds/apiextensions.crossplane.io_compositeresourcedefinitions.yaml",
            "https://github.com/crossplane/crossplane/blob/master/cluster/crds/apiextensions.crossplane.io_compositionrevisions.yaml",
            "https://github.com/crossplane/crossplane/blob/master/cluster/crds/apiextensions.crossplane.io_compositions.yaml",
            "https://github.com/crossplane/crossplane/blob/master/cluster/crds/pkg.crossplane.io_configurationrevisions.yaml",
            "https://github.com/crossplane/crossplane/blob/master/cluster/crds/pkg.crossplane.io_configurations.yaml",
            "https://github.com/crossplane/crossplane/blob/master/cluster/crds/pkg.crossplane.io_controllerconfigs.yaml",
            "https://github.com/crossplane/crossplane/blob/master/cluster/crds/pkg.crossplane.io_locks.yaml",
            "https://github.com/crossplane/crossplane/blob/master/cluster/crds/pkg.crossplane.io_providerrevisions.yaml",
            "https://github.com/crossplane/crossplane/blob/master/cluster/crds/pkg.crossplane.io_providers.yaml",
            "https://github.com/crossplane/crossplane/blob/master/cluster/crds/secrets.crossplane.io_storeconfigs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "CrunchyData/postgres-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/CrunchyData/postgres-operator/blob/master/config/crd/bases/postgres-operator.crunchydata.com_postgresclusters.yaml",
        ],
    },
    UpstreamSource {
        project_name: "cryostatio/cryostat-operator",
        license: UPL_V1,
        urls: &[
            "https://github.com/cryostatio/cryostat-operator/blob/main/config/crd/bases/operator.cryostat.io_cryostats.yaml",
        ],
    },
    UpstreamSource {
        project_name: "ctron/ditto-operator",
        license: EPL_V2,
        urls: &[
            "https://github.com/ctron/ditto-operator/blob/main/helm/ditto-operator/crds/ditto.yaml",
        ],
    },
    UpstreamSource {
        project_name: "ctron/hawkbit-operator",
        license: EPL_V2,
        urls: &[
            "https://github.com/ctron/hawkbit-operator/blob/main/crds/hawkbit.crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "dexidp/dex",
        license: APACHE_V2,
        urls: &[
            "https://github.com/dexidp/dex/blob/master/scripts/manifests/crds/authcodes.yaml",
            "https://github.com/dexidp/dex/blob/master/scripts/manifests/crds/authrequests.yaml",
            "https://github.com/dexidp/dex/blob/master/scripts/manifests/crds/connectors.yaml",
            "https://github.com/dexidp/dex/blob/master/scripts/manifests/crds/devicerequests.yaml",
            "https://github.com/dexidp/dex/blob/master/scripts/manifests/crds/devicetokens.yaml",
            "https://github.com/dexidp/dex/blob/master/scripts/manifests/crds/oauth2clients.yaml",
            "https://github.com/dexidp/dex/blob/master/scripts/manifests/crds/offlinesessionses.yaml",
            "https://github.com/dexidp/dex/blob/master/scripts/manifests/crds/passwords.yaml",
            "https://github.com/dexidp/dex/blob/master/scripts/manifests/crds/refreshtokens.yaml",
            "https://github.com/dexidp/dex/blob/master/scripts/manifests/crds/signingkeies.yaml",
        ],
    },
    UpstreamSource {
        project_name: "digitalis-io/vals-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/digitalis-io/vals-operator/blob/main/config/crd/bases/digitalis.io_dbsecrets.yaml",
            "https://github.com/digitalis-io/vals-operator/blob/main/config/crd/bases/digitalis.io_valssecrets.yaml",
        ],
    },
    UpstreamSource {
        project_name: "dmesser/cockroachdb-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/dmesser/cockroachdb-operator/blob/main/config/crd/bases/charts.operatorhub.io_cockroachdbs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "eclipse-che/che-operator",
        license: EPL_V2,
        urls: &[
            "https://github.com/eclipse-che/che-operator/blob/main/config/crd/bases/org.eclipse.che_checlusters.yaml",
        ],
    },
    UpstreamSource {
        project_name: "elastic/cloud-on-k8s",
        license: ELASTIC,
        urls: &[
            "https://github.com/elastic/cloud-on-k8s/blob/main/config/crds/v1/bases/agent.k8s.elastic.co_agents.yaml",
            "https://github.com/elastic/cloud-on-k8s/blob/main/config/crds/v1/bases/apm.k8s.elastic.co_apmservers.yaml",
            "https://github.com/elastic/cloud-on-k8s/blob/main/config/crds/v1/bases/autoscaling.k8s.elastic.co_elasticsearchautoscalers.yaml",
            "https://github.com/elastic/cloud-on-k8s/blob/main/config/crds/v1/bases/beat.k8s.elastic.co_beats.yaml",
            "https://github.com/elastic/cloud-on-k8s/blob/main/config/crds/v1/bases/elasticsearch.k8s.elastic.co_elasticsearches.yaml",
            "https://github.com/elastic/cloud-on-k8s/blob/main/config/crds/v1/bases/enterprisesearch.k8s.elastic.co_enterprisesearches.yaml",
            "https://github.com/elastic/cloud-on-k8s/blob/main/config/crds/v1/bases/kibana.k8s.elastic.co_kibanas.yaml",
            "https://github.com/elastic/cloud-on-k8s/blob/main/config/crds/v1/bases/maps.k8s.elastic.co_elasticmapsservers.yaml",
        ],
    },
    UpstreamSource {
        project_name: "emissary-ingress/emissary",
        license: MIT,
        urls: &[
            "https://github.com/emissary-ingress/emissary/blob/master/pkg/api/getambassador.io/crds.yaml",
        ],
    },
    UpstreamSource {
        project_name: "external-secrets/external-secrets",
        license: APACHE_V2,
        urls: &[
            "https://github.com/external-secrets/external-secrets/blob/main/config/crds/bases/external-secrets.io_clusterexternalsecrets.yaml",
            "https://github.com/external-secrets/external-secrets/blob/main/config/crds/bases/external-secrets.io_clustersecretstores.yaml",
            "https://github.com/external-secrets/external-secrets/blob/main/config/crds/bases/external-secrets.io_externalsecrets.yaml",
            "https://github.com/external-secrets/external-secrets/blob/main/config/crds/bases/external-secrets.io_secretstores.yaml",
        ],
    },
    UpstreamSource {
        project_name: "Flagsmith/flagsmith-operator",
        license: FLAGSMITH,
        urls: &[
            "https://github.com/Flagsmith/flagsmith-operator/blob/master/config/crd/bases/charts.flagsmith.com_flagsmiths.yaml",
        ],
    },
    UpstreamSource {
        project_name: "fluid-cloudnative/fluid",
        license: APACHE_V2,
        urls: &[
            "https://github.com/fluid-cloudnative/fluid/blob/master/config/crd/bases/data.fluid.io_alluxioruntimes.yaml",
            "https://github.com/fluid-cloudnative/fluid/blob/master/config/crd/bases/data.fluid.io_databackups.yaml",
            "https://github.com/fluid-cloudnative/fluid/blob/master/config/crd/bases/data.fluid.io_dataloads.yaml",
            "https://github.com/fluid-cloudnative/fluid/blob/master/config/crd/bases/data.fluid.io_datasets.yaml",
            "https://github.com/fluid-cloudnative/fluid/blob/master/config/crd/bases/data.fluid.io_goosefsruntimes.yaml",
            "https://github.com/fluid-cloudnative/fluid/blob/master/config/crd/bases/data.fluid.io_jindoruntimes.yaml",
            "https://github.com/fluid-cloudnative/fluid/blob/master/config/crd/bases/data.fluid.io_juicefsruntimes.yaml",
            "https://github.com/fluid-cloudnative/fluid/blob/master/config/crd/bases/data.fluid.io_thinruntimeprofiles.yaml",
            "https://github.com/fluid-cloudnative/fluid/blob/master/config/crd/bases/data.fluid.io_thinruntimes.yaml",
        ],
    },
    UpstreamSource {
        project_name: "fluxcd/flagger",
        license: APACHE_V2,
        urls: &[
            "https://github.com/fluxcd/flagger/blob/main/artifacts/flagger/crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "fluxcd/helm-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/fluxcd/helm-controller/blob/main/config/crd/bases/helm.toolkit.fluxcd.io_helmreleases.yaml",
        ],
    },
    UpstreamSource {
        project_name: "fluxcd/image-automation-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/fluxcd/image-automation-controller/blob/main/config/crd/bases/image.toolkit.fluxcd.io_imageupdateautomations.yaml",
        ],
    },
    UpstreamSource {
        project_name: "fluxcd/image-reflector-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/fluxcd/image-reflector-controller/blob/main/config/crd/bases/image.toolkit.fluxcd.io_imagepolicies.yaml",
            "https://github.com/fluxcd/image-reflector-controller/blob/main/config/crd/bases/image.toolkit.fluxcd.io_imagerepositories.yaml",
        ],
    },
    UpstreamSource {
        project_name: "fluxcd/kustomize-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/fluxcd/kustomize-controller/blob/main/config/crd/bases/kustomize.toolkit.fluxcd.io_kustomizations.yaml",
        ],
    },
    UpstreamSource {
        project_name: "fluxcd/notification-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/fluxcd/notification-controller/blob/main/config/crd/bases/notification.toolkit.fluxcd.io_alerts.yaml",
            "https://github.com/fluxcd/notification-controller/blob/main/config/crd/bases/notification.toolkit.fluxcd.io_providers.yaml",
            "https://github.com/fluxcd/notification-controller/blob/main/config/crd/bases/notification.toolkit.fluxcd.io_receivers.yaml",
        ],
    },
    UpstreamSource {
        project_name: "fluxcd/source-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/fluxcd/source-controller/blob/main/config/crd/bases/source.toolkit.fluxcd.io_buckets.yaml",
            "https://github.com/fluxcd/source-controller/blob/main/config/crd/bases/source.toolkit.fluxcd.io_gitrepositories.yaml",
            "https://github.com/fluxcd/source-controller/blob/main/config/crd/bases/source.toolkit.fluxcd.io_helmcharts.yaml",
            "https://github.com/fluxcd/source-controller/blob/main/config/crd/bases/source.toolkit.fluxcd.io_helmrepositories.yaml",
            "https://github.com/fluxcd/source-controller/blob/main/config/crd/bases/source.toolkit.fluxcd.io_ocirepositories.yaml",
        ],
    },
    UpstreamSource {
        project_name: "flux-framework/flux-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/flux-framework/flux-operator/blob/main/config/crd/bases/flux-framework.org_miniclusters.yaml",
        ],
    },
    UpstreamSource {
        project_name: "fossul/fossul",
        license: APACHE_V2,
        urls: &[
            "https://github.com/fossul/fossul/blob/master/operator/config/crd/bases/fossul.io_backupconfigs.yaml",
            "https://github.com/fossul/fossul/blob/master/operator/config/crd/bases/fossul.io_backups.yaml",
            "https://github.com/fossul/fossul/blob/master/operator/config/crd/bases/fossul.io_backupschedules.yaml",
            "https://github.com/fossul/fossul/blob/master/operator/config/crd/bases/fossul.io_fossuls.yaml",
            "https://github.com/fossul/fossul/blob/master/operator/config/crd/bases/fossul.io_restores.yaml",
        ],
    },
    UpstreamSource {
        project_name: "furiko-io/furiko",
        license: APACHE_V2,
        urls: &[
            "https://github.com/furiko-io/furiko/blob/main/config/crd/bases/execution.furiko.io_jobconfigs.yaml",
            "https://github.com/furiko-io/furiko/blob/main/config/crd/bases/execution.furiko.io_jobs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "gitlab-org/cloud-native/gitlab-operator",
        license: APACHE_V2,
        urls: &[
            "https://gitlab.com/gitlab-org/cloud-native/gitlab-operator/-/blob/master/config/crd/bases/apps.gitlab.com_gitlabs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "gitlab-org/gl-openshift/gitlab-runner-operator",
        license: APACHE_V2,
        urls: &[
            "https://gitlab.com/gitlab-org/gl-openshift/gitlab-runner-operator/-/blob/master/config/crd_k8s/bases/apps.gitlab.com_runners.yaml",
        ],
    },
    UpstreamSource {
        project_name: "GoogleCloudPlatform/spark-on-k8s-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/GoogleCloudPlatform/spark-on-k8s-operator/blob/master/charts/spark-operator-chart/crds/sparkoperator.k8s.io_scheduledsparkapplications.yaml",
            "https://github.com/GoogleCloudPlatform/spark-on-k8s-operator/blob/master/charts/spark-operator-chart/crds/sparkoperator.k8s.io_sparkapplications.yaml",
        ],
    },
    UpstreamSource {
        project_name: "grafana-operator/grafana-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/grafana-operator/grafana-operator/blob/master/config/crd/bases/grafana.integreatly.org_grafanadashboards.yaml",
            "https://github.com/grafana-operator/grafana-operator/blob/master/config/crd/bases/grafana.integreatly.org_grafanadatasources.yaml",
            "https://github.com/grafana-operator/grafana-operator/blob/master/config/crd/bases/grafana.integreatly.org_grafanafolders.yaml",
            "https://github.com/grafana-operator/grafana-operator/blob/master/config/crd/bases/grafana.integreatly.org_grafanas.yaml",
        ],
    },
    UpstreamSource {
        project_name: "grafana/loki",
        license: AGPL_V3_ONLY,
        urls: &[
            "https://github.com/grafana/loki/blob/main/operator/config/crd/bases/config.grafana.com_projectconfigs.yaml",
            "https://github.com/grafana/loki/blob/main/operator/config/crd/bases/loki.grafana.com_alertingrules.yaml",
            "https://github.com/grafana/loki/blob/main/operator/config/crd/bases/loki.grafana.com_lokistacks.yaml",
            "https://github.com/grafana/loki/blob/main/operator/config/crd/bases/loki.grafana.com_recordingrules.yaml",
            "https://github.com/grafana/loki/blob/main/operator/config/crd/bases/loki.grafana.com_rulerconfigs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "gravitational/teleport",
        license: APACHE_V2,
        urls: &[
            "https://github.com/gravitational/teleport/blob/master/integrations/operator/config/crd/bases/resources.teleport.dev_githubconnectors.yaml",
            "https://github.com/gravitational/teleport/blob/master/integrations/operator/config/crd/bases/resources.teleport.dev_loginrules.yaml",
            "https://github.com/gravitational/teleport/blob/master/integrations/operator/config/crd/bases/resources.teleport.dev_oidcconnectors.yaml",
            "https://github.com/gravitational/teleport/blob/master/integrations/operator/config/crd/bases/resources.teleport.dev_oktaimportrules.yaml",
            "https://github.com/gravitational/teleport/blob/master/integrations/operator/config/crd/bases/resources.teleport.dev_provisiontokens.yaml",
            "https://github.com/gravitational/teleport/blob/master/integrations/operator/config/crd/bases/resources.teleport.dev_roles.yaml",
            "https://github.com/gravitational/teleport/blob/master/integrations/operator/config/crd/bases/resources.teleport.dev_samlconnectors.yaml",
            "https://github.com/gravitational/teleport/blob/master/integrations/operator/config/crd/bases/resources.teleport.dev_users.yaml",
        ],
    },
    UpstreamSource {
        project_name: "hazelcast/hazelcast-platform-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/hazelcast/hazelcast-platform-operator/blob/main/config/crd/bases/hazelcast.com_cronhotbackups.yaml",
            "https://github.com/hazelcast/hazelcast-platform-operator/blob/main/config/crd/bases/hazelcast.com_hazelcasts.yaml",
            "https://github.com/hazelcast/hazelcast-platform-operator/blob/main/config/crd/bases/hazelcast.com_hotbackups.yaml",
            "https://github.com/hazelcast/hazelcast-platform-operator/blob/main/config/crd/bases/hazelcast.com_managementcenters.yaml",
            "https://github.com/hazelcast/hazelcast-platform-operator/blob/main/config/crd/bases/hazelcast.com_maps.yaml",
            "https://github.com/hazelcast/hazelcast-platform-operator/blob/main/config/crd/bases/hazelcast.com_wanreplications.yaml",
        ],
    },
    UpstreamSource {
        project_name: "Hyperfoil/horreum-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/Hyperfoil/horreum-operator/blob/master/config/crd/bases/hyperfoil.io_horreums.yaml",
        ],
    },
    UpstreamSource {
        project_name: "Hyperfoil/hyperfoil-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/Hyperfoil/hyperfoil-operator/blob/master/config/crd/bases/hyperfoil.io_hyperfoils.yaml",
        ],
    },
    UpstreamSource {
        project_name: "IBM/varnish-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/IBM/varnish-operator/blob/main/config/crd/bases/caching.ibm.com_varnishclusters.yaml",
        ],
    },
    UpstreamSource {
        project_name: "infinispan/infinispan-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/infinispan/infinispan-operator/blob/main/config/crd/bases/infinispan.org_infinispans.yaml",
            "https://github.com/infinispan/infinispan-operator/blob/main/config/crd/bases/infinispan.org_backups.yaml",
            "https://github.com/infinispan/infinispan-operator/blob/main/config/crd/bases/infinispan.org_batches.yaml",
            "https://github.com/infinispan/infinispan-operator/blob/main/config/crd/bases/infinispan.org_caches.yaml",
            "https://github.com/infinispan/infinispan-operator/blob/main/config/crd/bases/infinispan.org_restores.yaml",
        ],
    },
    UpstreamSource {
        project_name: "istio/istio",
        license: APACHE_V2,
        urls: &[
            "https://github.com/istio/istio/blob/master/manifests/charts/base/crds/crd-all.gen.yaml",
            "https://github.com/istio/istio/blob/master/manifests/charts/base/crds/crd-operator.yaml",
        ],
    },
    UpstreamSource {
        project_name: "jaegertracing/jaeger-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/jaegertracing/jaeger-operator/blob/main/config/crd/bases/jaegertracing.io_jaegers.yaml",
        ],
    },
    UpstreamSource {
        project_name: "k8gb-io/k8gb",
        license: APACHE_V2,
        urls: &[
            "https://github.com/k8gb-io/k8gb/blob/master/chart/k8gb/crd/dns-endpoint-crd-manifest.yaml",
            "https://github.com/k8gb-io/k8gb/blob/master/chart/k8gb/crd/k8gb.absa.oss_gslbs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "karmada-io/karmada",
        license: APACHE_V2,
        urls: &[
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/autoscaling/autoscaling.karmada.io_cronfederatedhpas.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/autoscaling/autoscaling.karmada.io_federatedhpas.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/config/config.karmada.io_resourceinterpretercustomizations.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/config/config.karmada.io_resourceinterpreterwebhookconfigurations.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/multicluster/multicluster.x-k8s.io_serviceexports.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/multicluster/multicluster.x-k8s.io_serviceimports.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/networking/networking.karmada.io_multiclusteringresses.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/networking/networking.karmada.io_multiclusterservices.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/policy/policy.karmada.io_clusteroverridepolicies.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/policy/policy.karmada.io_clusterpropagationpolicies.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/policy/policy.karmada.io_federatedresourcequotas.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/policy/policy.karmada.io_overridepolicies.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/policy/policy.karmada.io_propagationpolicies.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/work/work.karmada.io_clusterresourcebindings.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/work/work.karmada.io_resourcebindings.yaml",
            "https://github.com/karmada-io/karmada/blob/master/charts/karmada/_crds/bases/work/work.karmada.io_works.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kedacore/keda",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kedacore/keda/blob/main/config/crd/bases/keda.sh_clustertriggerauthentications.yaml",
            "https://github.com/kedacore/keda/blob/main/config/crd/bases/keda.sh_scaledjobs.yaml",
            "https://github.com/kedacore/keda/blob/main/config/crd/bases/keda.sh_scaledobjects.yaml",
            "https://github.com/kedacore/keda/blob/main/config/crd/bases/keda.sh_triggerauthentications.yaml",
        ],
    },
    UpstreamSource {
        project_name: "keycloak/keycloak-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/keycloak/keycloak-operator/blob/main/deploy/crds/keycloak.org_keycloakbackups_crd.yaml",
            "https://github.com/keycloak/keycloak-operator/blob/main/deploy/crds/keycloak.org_keycloakclients_crd.yaml",
            "https://github.com/keycloak/keycloak-operator/blob/main/deploy/crds/keycloak.org_keycloakrealms_crd.yaml",
            "https://github.com/keycloak/keycloak-operator/blob/main/deploy/crds/keycloak.org_keycloaks_crd.yaml",
            "https://github.com/keycloak/keycloak-operator/blob/main/deploy/crds/keycloak.org_keycloakusers_crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kiali/kiali-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kiali/kiali-operator/blob/master/crd-docs/crd/kiali.io_kialis.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kiegroup/kogito-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kiegroup/kogito-operator/blob/main/config/crd/app/bases/app.kiegroup.org_kogitobuilds.yaml",
            "https://github.com/kiegroup/kogito-operator/blob/main/config/crd/app/bases/app.kiegroup.org_kogitoinfras.yaml",
            "https://github.com/kiegroup/kogito-operator/blob/main/config/crd/app/bases/app.kiegroup.org_kogitoruntimes.yaml",
            "https://github.com/kiegroup/kogito-operator/blob/main/config/crd/app/bases/app.kiegroup.org_kogitosupportingservices.yaml",
        ],
    },
    UpstreamSource {
        project_name: "knative/operator",
        license: APACHE_V2,
        urls: &[
            "https://raw.githubusercontent.com/knative/operator/main/config/crd/bases/operator.knative.dev_knativeeventings.yaml",
            "https://raw.githubusercontent.com/knative/operator/main/config/crd/bases/operator.knative.dev_knativeservings.yaml",
        ],
    },
    UpstreamSource {
        project_name: "koordinator-sh/koordinator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/koordinator-sh/koordinator/blob/main/config/crd/bases/config.koordinator.sh_clustercolocationprofiles.yaml",
            "https://github.com/koordinator-sh/koordinator/blob/main/config/crd/bases/scheduling.koordinator.sh_devices.yaml",
            "https://github.com/koordinator-sh/koordinator/blob/main/config/crd/bases/scheduling.koordinator.sh_podmigrationjobs.yaml",
            "https://github.com/koordinator-sh/koordinator/blob/main/config/crd/bases/scheduling.koordinator.sh_reservations.yaml",
            "https://github.com/koordinator-sh/koordinator/blob/main/config/crd/bases/scheduling.sigs.k8s.io_elasticquotas.yaml",
            "https://github.com/koordinator-sh/koordinator/blob/main/config/crd/bases/scheduling.sigs.k8s.io_podgroups.yaml",
            "https://github.com/koordinator-sh/koordinator/blob/main/config/crd/bases/slo.koordinator.sh_nodemetrics.yaml",
            "https://github.com/koordinator-sh/koordinator/blob/main/config/crd/bases/slo.koordinator.sh_nodeslos.yaml",
        ],
    },
    UpstreamSource {
        project_name: "ktsstudio/mirrors",
        license: APACHE_V2,
        urls: &[
            "https://github.com/ktsstudio/mirrors/blob/main/config/crd/bases/mirrors.kts.studio_secretmirrors.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kube-logging/logging-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kube-logging/logging-operator/blob/master/charts/logging-operator/crds/logging-extensions.banzaicloud.io_eventtailers.yaml",
            "https://github.com/kube-logging/logging-operator/blob/master/charts/logging-operator/crds/logging-extensions.banzaicloud.io_hosttailers.yaml",
            "https://github.com/kube-logging/logging-operator/blob/master/charts/logging-operator/crds/logging.banzaicloud.io_clusterflows.yaml",
            "https://github.com/kube-logging/logging-operator/blob/master/charts/logging-operator/crds/logging.banzaicloud.io_clusteroutputs.yaml",
            "https://github.com/kube-logging/logging-operator/blob/master/charts/logging-operator/crds/logging.banzaicloud.io_flows.yaml",
            "https://github.com/kube-logging/logging-operator/blob/master/charts/logging-operator/crds/logging.banzaicloud.io_fluentbitagents.yaml",
            "https://github.com/kube-logging/logging-operator/blob/master/charts/logging-operator/crds/logging.banzaicloud.io_loggings.yaml",
            "https://github.com/kube-logging/logging-operator/blob/master/charts/logging-operator/crds/logging.banzaicloud.io_nodeagents.yaml",
            "https://github.com/kube-logging/logging-operator/blob/master/charts/logging-operator/crds/logging.banzaicloud.io_outputs.yaml",
            "https://github.com/kube-logging/logging-operator/blob/master/charts/logging-operator/crds/logging.banzaicloud.io_syslogngclusterflows.yaml",
            "https://github.com/kube-logging/logging-operator/blob/master/charts/logging-operator/crds/logging.banzaicloud.io_syslogngclusteroutputs.yaml",
            "https://github.com/kube-logging/logging-operator/blob/master/charts/logging-operator/crds/logging.banzaicloud.io_syslogngflows.yaml",
            "https://github.com/kube-logging/logging-operator/blob/master/charts/logging-operator/crds/logging.banzaicloud.io_syslogngoutputs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubean-io/kubean",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubean-io/kubean/blob/main/charts/kubean/crds/kubean.io_clusteroperations.yaml",
            "https://github.com/kubean-io/kubean/blob/main/charts/kubean/crds/kubean.io_clusters.yaml",
            "https://github.com/kubean-io/kubean/blob/main/charts/kubean/crds/kubean.io_localartifactsets.yaml",
            "https://github.com/kubean-io/kubean/blob/main/charts/kubean/crds/kubean.io_manifests.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubedl-io/kubedl",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/apps.kubedl.io_crons.yaml",
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/cache.kubedl.io_cachebackends.yaml",
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/inference.kubedl.io_elasticbatchjobs.yaml",
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/model.kubedl.io_models.yaml",
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/model.kubedl.io_modelversions.yaml",
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/notebook.kubedl.io_notebooks.yaml",
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/serving.kubedl.io_inferences.yaml",
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/training.kubedl.io_elasticdljobs.yaml",
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/training.kubedl.io_marsjobs.yaml",
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/training.kubedl.io_mpijobs.yaml",
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/training.kubedl.io_pytorchjobs.yaml",
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/training.kubedl.io_tfjobs.yaml",
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/training.kubedl.io_xdljobs.yaml",
            "https://github.com/kubedl-io/kubedl/blob/master/config/crd/bases/training.kubedl.io_xgboostjobs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubeedge/kubeedge",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubeedge/kubeedge/blob/master/manifests/charts/cloudcore/crds/apps_v1alpha1_edgeapplication.yaml",
            "https://github.com/kubeedge/kubeedge/blob/master/manifests/charts/cloudcore/crds/apps_v1alpha1_nodegroup.yaml",
            "https://github.com/kubeedge/kubeedge/blob/master/manifests/charts/cloudcore/crds/cluster_objectsync_v1alpha1.yaml",
            "https://github.com/kubeedge/kubeedge/blob/master/manifests/charts/cloudcore/crds/devices_v1alpha2_device.yaml",
            "https://github.com/kubeedge/kubeedge/blob/master/manifests/charts/cloudcore/crds/devices_v1alpha2_devicemodel.yaml",
            "https://github.com/kubeedge/kubeedge/blob/master/manifests/charts/cloudcore/crds/objectsync_v1alpha1.yaml",
            "https://github.com/kubeedge/kubeedge/blob/master/manifests/charts/cloudcore/crds/operations_v1alpha1_nodeupgradejob.yaml",
            "https://github.com/kubeedge/kubeedge/blob/master/manifests/charts/cloudcore/crds/router_v1_rule.yaml",
            "https://github.com/kubeedge/kubeedge/blob/master/manifests/charts/cloudcore/crds/router_v1_ruleEndpoint.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/about-api",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/about-api/blob/master/config/crd/about.k8s.io_clusterproperties.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/aws-load-balancer-controller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/aws-load-balancer-controller/blob/main/config/crd/bases/elbv2.k8s.aws_ingressclassparams.yaml",
            "https://github.com/kubernetes-sigs/aws-load-balancer-controller/blob/main/config/crd/bases/elbv2.k8s.aws_targetgroupbindings.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/boskos",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/boskos/blob/master/deployments/base/crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/cluster-api",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/cluster-api/blob/main/config/crd/bases/addons.cluster.x-k8s.io_clusterresourcesetbindings.yaml",
            "https://github.com/kubernetes-sigs/cluster-api/blob/main/config/crd/bases/addons.cluster.x-k8s.io_clusterresourcesets.yaml",
            "https://github.com/kubernetes-sigs/cluster-api/blob/main/config/crd/bases/cluster.x-k8s.io_clusterclasses.yaml",
            "https://github.com/kubernetes-sigs/cluster-api/blob/main/config/crd/bases/cluster.x-k8s.io_clusters.yaml",
            "https://github.com/kubernetes-sigs/cluster-api/blob/main/config/crd/bases/cluster.x-k8s.io_machinedeployments.yaml",
            "https://github.com/kubernetes-sigs/cluster-api/blob/main/config/crd/bases/cluster.x-k8s.io_machinehealthchecks.yaml",
            "https://github.com/kubernetes-sigs/cluster-api/blob/main/config/crd/bases/cluster.x-k8s.io_machinepools.yaml",
            "https://github.com/kubernetes-sigs/cluster-api/blob/main/config/crd/bases/cluster.x-k8s.io_machines.yaml",
            "https://github.com/kubernetes-sigs/cluster-api/blob/main/config/crd/bases/cluster.x-k8s.io_machinesets.yaml",
            "https://github.com/kubernetes-sigs/cluster-api/blob/main/config/crd/bases/ipam.cluster.x-k8s.io_ipaddressclaims.yaml",
            "https://github.com/kubernetes-sigs/cluster-api/blob/main/config/crd/bases/ipam.cluster.x-k8s.io_ipaddresses.yaml",
            "https://github.com/kubernetes-sigs/cluster-api/blob/main/config/crd/bases/runtime.cluster.x-k8s.io_extensionconfigs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/cluster-api-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/cluster-api-operator/blob/main/config/crd/bases/operator.cluster.x-k8s.io_addonproviders.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-operator/blob/main/config/crd/bases/operator.cluster.x-k8s.io_bootstrapproviders.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-operator/blob/main/config/crd/bases/operator.cluster.x-k8s.io_controlplaneproviders.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-operator/blob/main/config/crd/bases/operator.cluster.x-k8s.io_coreproviders.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-operator/blob/main/config/crd/bases/operator.cluster.x-k8s.io_infrastructureproviders.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/cluster-api-provider-ibmcloud",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/cluster-api-provider-ibmcloud/blob/main/config/crd/bases/infrastructure.cluster.x-k8s.io_ibmpowervsclusters.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-ibmcloud/blob/main/config/crd/bases/infrastructure.cluster.x-k8s.io_ibmpowervsclustertemplates.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-ibmcloud/blob/main/config/crd/bases/infrastructure.cluster.x-k8s.io_ibmpowervsimages.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-ibmcloud/blob/main/config/crd/bases/infrastructure.cluster.x-k8s.io_ibmpowervsmachines.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-ibmcloud/blob/main/config/crd/bases/infrastructure.cluster.x-k8s.io_ibmpowervsmachinetemplates.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-ibmcloud/blob/main/config/crd/bases/infrastructure.cluster.x-k8s.io_ibmvpcclusters.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-ibmcloud/blob/main/config/crd/bases/infrastructure.cluster.x-k8s.io_ibmvpcmachines.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-ibmcloud/blob/main/config/crd/bases/infrastructure.cluster.x-k8s.io_ibmvpcmachinetemplates.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/cluster-api-provider-kubevirt",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/cluster-api-provider-kubevirt/blob/main/config/crd/bases/infrastructure.cluster.x-k8s.io_kubevirtclusters.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-kubevirt/blob/main/config/crd/bases/infrastructure.cluster.x-k8s.io_kubevirtclustertemplates.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-kubevirt/blob/main/config/crd/bases/infrastructure.cluster.x-k8s.io_kubevirtmachines.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-kubevirt/blob/main/config/crd/bases/infrastructure.cluster.x-k8s.io_kubevirtmachinetemplates.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/cluster-api-provider-vsphere",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/cluster-api-provider-vsphere/blob/main/config/default/crd/bases/infrastructure.cluster.x-k8s.io_haproxyloadbalancers.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-vsphere/blob/main/config/default/crd/bases/infrastructure.cluster.x-k8s.io_vsphereclusteridentities.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-vsphere/blob/main/config/default/crd/bases/infrastructure.cluster.x-k8s.io_vsphereclusters.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-vsphere/blob/main/config/default/crd/bases/infrastructure.cluster.x-k8s.io_vsphereclustertemplates.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-vsphere/blob/main/config/default/crd/bases/infrastructure.cluster.x-k8s.io_vspheredeploymentzones.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-vsphere/blob/main/config/default/crd/bases/infrastructure.cluster.x-k8s.io_vspherefailuredomains.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-vsphere/blob/main/config/default/crd/bases/infrastructure.cluster.x-k8s.io_vspheremachines.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-vsphere/blob/main/config/default/crd/bases/infrastructure.cluster.x-k8s.io_vspheremachinetemplates.yaml",
            "https://github.com/kubernetes-sigs/cluster-api-provider-vsphere/blob/main/config/default/crd/bases/infrastructure.cluster.x-k8s.io_vspherevms.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/gateway-api",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/gateway-api/blob/main/config/crd/experimental/gateway.networking.k8s.io_gatewayclasses.yaml",
            "https://github.com/kubernetes-sigs/gateway-api/blob/main/config/crd/experimental/gateway.networking.k8s.io_gateways.yaml",
            "https://github.com/kubernetes-sigs/gateway-api/blob/main/config/crd/experimental/gateway.networking.k8s.io_grpcroutes.yaml",
            "https://github.com/kubernetes-sigs/gateway-api/blob/main/config/crd/experimental/gateway.networking.k8s.io_httproutes.yaml",
            "https://github.com/kubernetes-sigs/gateway-api/blob/main/config/crd/experimental/gateway.networking.k8s.io_referencegrants.yaml",
            "https://github.com/kubernetes-sigs/gateway-api/blob/main/config/crd/experimental/gateway.networking.k8s.io_tcproutes.yaml",
            "https://github.com/kubernetes-sigs/gateway-api/blob/main/config/crd/experimental/gateway.networking.k8s.io_tlsroutes.yaml",
            "https://github.com/kubernetes-sigs/gateway-api/blob/main/config/crd/experimental/gateway.networking.k8s.io_udproutes.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/hierarchical-namespaces",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/hierarchical-namespaces/blob/master/config/crd/bases/hnc.x-k8s.io_hierarchicalresourcequotas.yaml",
            "https://github.com/kubernetes-sigs/hierarchical-namespaces/blob/master/config/crd/bases/hnc.x-k8s.io_hierarchyconfigurations.yaml",
            "https://github.com/kubernetes-sigs/hierarchical-namespaces/blob/master/config/crd/bases/hnc.x-k8s.io_hncconfigurations.yaml",
            "https://github.com/kubernetes-sigs/hierarchical-namespaces/blob/master/config/crd/bases/hnc.x-k8s.io_subnamespaceanchors.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/jobset",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/jobset/blob/main/config/components/crd/bases/jobset.x-k8s.io_jobsets.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/kueue",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/kueue/blob/main/config/components/crd/bases/kueue.x-k8s.io_admissionchecks.yaml",
            "https://github.com/kubernetes-sigs/kueue/blob/main/config/components/crd/bases/kueue.x-k8s.io_clusterqueues.yaml",
            "https://github.com/kubernetes-sigs/kueue/blob/main/config/components/crd/bases/kueue.x-k8s.io_localqueues.yaml",
            "https://github.com/kubernetes-sigs/kueue/blob/main/config/components/crd/bases/kueue.x-k8s.io_resourceflavors.yaml",
            "https://github.com/kubernetes-sigs/kueue/blob/main/config/components/crd/bases/kueue.x-k8s.io_workloads.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/network-policy-api",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/network-policy-api/blob/main/config/crd/policy.networking.k8s.io_adminnetworkpolicies.yaml",
            "https://github.com/kubernetes-sigs/network-policy-api/blob/main/config/crd/policy.networking.k8s.io_baselineadminnetworkpolicies.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/node-feature-discovery-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/node-feature-discovery-operator/blob/master/config/crd/bases/nfd.kubernetes.io_nodefeaturediscoveries.yaml",
            "https://github.com/kubernetes-sigs/node-feature-discovery-operator/blob/master/config/crd/bases/nfd.kubernetes.io_v1alpha1_nodefeaturerules.yaml",
            "https://github.com/kubernetes-sigs/node-feature-discovery-operator/blob/master/config/crd/bases/node.k8s.io_v1alpha1_noderesourcetopologies.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/security-profiles-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/security-profiles-operator/blob/main/deploy/base-crds/crds/apparmorprofile.yaml",
            "https://github.com/kubernetes-sigs/security-profiles-operator/blob/main/deploy/base-crds/crds/profilebinding.yaml",
            "https://github.com/kubernetes-sigs/security-profiles-operator/blob/main/deploy/base-crds/crds/profilerecording.yaml",
            "https://github.com/kubernetes-sigs/security-profiles-operator/blob/main/deploy/base-crds/crds/seccompprofile.yaml",
            "https://github.com/kubernetes-sigs/security-profiles-operator/blob/main/deploy/base-crds/crds/securityprofilenodestatus.yaml",
            "https://github.com/kubernetes-sigs/security-profiles-operator/blob/main/deploy/base-crds/crds/securityprofilesoperatordaemon.yaml",
            "https://github.com/kubernetes-sigs/security-profiles-operator/blob/main/deploy/base-crds/crds/selinuxpolicy.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/wg-policy-prototypes",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/wg-policy-prototypes/blob/master/policy-report/crd/v1alpha1/wgpolicyk8s.io_clusterpolicyreports.yaml",
            "https://github.com/kubernetes-sigs/wg-policy-prototypes/blob/master/policy-report/crd/v1alpha1/wgpolicyk8s.io_policyreports.yaml",
            "https://github.com/kubernetes-sigs/wg-policy-prototypes/blob/master/policy-report/crd/v1alpha2/wgpolicyk8s.io_clusterpolicyreports.yaml",
            "https://github.com/kubernetes-sigs/wg-policy-prototypes/blob/master/policy-report/crd/v1alpha2/wgpolicyk8s.io_policyreports.yaml",
            "https://github.com/kubernetes-sigs/wg-policy-prototypes/blob/master/policy-report/crd/v1beta1/wgpolicyk8s.io_clusterpolicyreports.yaml",
            "https://github.com/kubernetes-sigs/wg-policy-prototypes/blob/master/policy-report/crd/v1beta1/wgpolicyk8s.io_policyreports.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes-sigs/work-api",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes-sigs/work-api/blob/master/config/crd/multicluster.x-k8s.io_appliedworks.yaml",
            "https://github.com/kubernetes-sigs/work-api/blob/master/config/crd/multicluster.x-k8s.io_works.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubernetes/autoscaler",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubernetes/autoscaler/blob/master/vertical-pod-autoscaler/deploy/vpa-v1-crd-gen.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubeshop/testkube-operator",
        license: MIT,
        urls: &[
            "https://github.com/kubeshop/testkube-operator/blob/develop/config/crd/bases/executor.testkube.io_executors.yaml",
            "https://github.com/kubeshop/testkube-operator/blob/develop/config/crd/bases/executor.testkube.io_webhooks.yaml",
            "https://github.com/kubeshop/testkube-operator/blob/develop/config/crd/bases/tests.testkube.io_scripts.yaml",
            "https://github.com/kubeshop/testkube-operator/blob/develop/config/crd/bases/tests.testkube.io_testexecutions.yaml",
            "https://github.com/kubeshop/testkube-operator/blob/develop/config/crd/bases/tests.testkube.io_tests.yaml",
            "https://github.com/kubeshop/testkube-operator/blob/develop/config/crd/bases/tests.testkube.io_testsources.yaml",
            "https://github.com/kubeshop/testkube-operator/blob/develop/config/crd/bases/tests.testkube.io_testsuiteexecutions.yaml",
            "https://github.com/kubeshop/testkube-operator/blob/develop/config/crd/bases/tests.testkube.io_testsuites.yaml",
            "https://github.com/kubeshop/testkube-operator/blob/develop/config/crd/bases/tests.testkube.io_testtriggers.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kubevious/workload-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kubevious/workload-operator/blob/main/crds/workload-profile.yaml",
            "https://github.com/kubevious/workload-operator/blob/main/crds/workload.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kumahq/kuma",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_circuitbreakers.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_containerpatches.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_dataplaneinsights.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_dataplanes.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_externalservices.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_faultinjections.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_healthchecks.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshaccesslogs.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshcircuitbreakers.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshes.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshfaultinjections.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshgatewayconfigs.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshgatewayinstances.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshgatewayroutes.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshgateways.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshhealthchecks.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshhttproutes.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshinsights.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshloadbalancingstrategies.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshproxypatches.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshratelimits.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshretries.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshtcproutes.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshtimeouts.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshtraces.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_meshtrafficpermissions.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_proxytemplates.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_ratelimits.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_retries.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_serviceinsights.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_timeouts.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_trafficlogs.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_trafficpermissions.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_trafficroutes.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_traffictraces.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_virtualoutbounds.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_zoneegresses.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_zoneegressinsights.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_zoneingresses.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_zoneingressinsights.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_zoneinsights.yaml",
            "https://github.com/kumahq/kuma/blob/master/deployments/charts/kuma/crds/kuma.io_zones.yaml",
        ],
    },
    UpstreamSource {
        project_name: "kyverno/kyverno",
        license: APACHE_V2,
        urls: &[
            "https://github.com/kyverno/kyverno/blob/main/config/crds/kyverno.io_admissionreports.yaml",
            "https://github.com/kyverno/kyverno/blob/main/config/crds/kyverno.io_backgroundscanreports.yaml",
            "https://github.com/kyverno/kyverno/blob/main/config/crds/kyverno.io_cleanuppolicies.yaml",
            "https://github.com/kyverno/kyverno/blob/main/config/crds/kyverno.io_clusteradmissionreports.yaml",
            "https://github.com/kyverno/kyverno/blob/main/config/crds/kyverno.io_clusterbackgroundscanreports.yaml",
            "https://github.com/kyverno/kyverno/blob/main/config/crds/kyverno.io_clustercleanuppolicies.yaml",
            "https://github.com/kyverno/kyverno/blob/main/config/crds/kyverno.io_clusterpolicies.yaml",
            "https://github.com/kyverno/kyverno/blob/main/config/crds/kyverno.io_policies.yaml",
            "https://github.com/kyverno/kyverno/blob/main/config/crds/kyverno.io_policyexceptions.yaml",
            "https://github.com/kyverno/kyverno/blob/main/config/crds/kyverno.io_updaterequests.yaml",
        ],
    },
    UpstreamSource {
        project_name: "l7mp/stunner",
        license: MIT,
        urls: &[
            "https://github.com/l7mp/stunner/blob/main/deploy/manifests/static/stunner-crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "Lerentis/bitwarden-crd-operator",
        license: MIT,
        urls: &[
            "https://github.com/Lerentis/bitwarden-crd-operator/blob/main/charts/bitwarden-crd-operator/crds/bitwarden-secrets.yaml",
            "https://github.com/Lerentis/bitwarden-crd-operator/blob/main/charts/bitwarden-crd-operator/crds/bitwarden-templates.yaml",
            "https://github.com/Lerentis/bitwarden-crd-operator/blob/main/charts/bitwarden-crd-operator/crds/registry-credentials.yaml",
        ],
    },
    UpstreamSource {
        project_name: "lightbend/akka-cluster-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/lightbend/akka-cluster-operator/blob/master/deploy/crds/app_v1alpha1_akkacluster_crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "linki/cloudformation-operator",
        license: MIT,
        urls: &[
            "https://github.com/linki/cloudformation-operator/blob/master/config/crd/bases/cloudformation.linki.space_stacks.yaml",
        ],
    },
    UpstreamSource {
        project_name: "LinuxSuRen/api-testing",
        license: MIT,
        urls: &[
            "https://github.com/LinuxSuRen/api-testing/blob/master/operator/config/crd/bases/core.linuxsuren.github.com_atests.yaml",
        ],
    },
    UpstreamSource {
        project_name: "litmuschaos/chaos-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/litmuschaos/chaos-operator/blob/master/deploy/crds/chaosengine_crd.yaml",
            "https://github.com/litmuschaos/chaos-operator/blob/master/deploy/crds/chaosexperiment_crd.yaml",
            "https://github.com/litmuschaos/chaos-operator/blob/master/deploy/crds/chaosresults_crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "longhorn/longhorn",
        license: APACHE_V2,
        urls: &[
            "https://github.com/longhorn/longhorn/blob/master/deploy/longhorn.yaml",
        ],
    },
    UpstreamSource {
        project_name: "lukaszraczylo/jobs-manager-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/lukaszraczylo/jobs-manager-operator/blob/main/config/crd/bases/jobsmanager.raczylo.com_managedjobs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "m88i/nexus-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/m88i/nexus-operator/blob/main/config/crd/bases/apps.m88i.io_nexus.yaml",
        ],
    },
    UpstreamSource {
        project_name: "mariadb-operator/mariadb-operator",
        license: MIT,
        urls: &[
            "https://github.com/mariadb-operator/mariadb-operator/blob/main/config/crd/bases/mariadb.mmontes.io_backups.yaml",
            "https://github.com/mariadb-operator/mariadb-operator/blob/main/config/crd/bases/mariadb.mmontes.io_connections.yaml",
            "https://github.com/mariadb-operator/mariadb-operator/blob/main/config/crd/bases/mariadb.mmontes.io_databases.yaml",
            "https://github.com/mariadb-operator/mariadb-operator/blob/main/config/crd/bases/mariadb.mmontes.io_grants.yaml",
            "https://github.com/mariadb-operator/mariadb-operator/blob/main/config/crd/bases/mariadb.mmontes.io_mariadbs.yaml",
            "https://github.com/mariadb-operator/mariadb-operator/blob/main/config/crd/bases/mariadb.mmontes.io_restores.yaml",
            "https://github.com/mariadb-operator/mariadb-operator/blob/main/config/crd/bases/mariadb.mmontes.io_sqljobs.yaml",
            "https://github.com/mariadb-operator/mariadb-operator/blob/main/config/crd/bases/mariadb.mmontes.io_users.yaml",
        ],
    },
    UpstreamSource {
        project_name: "mattermost/mattermost-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/mattermost/mattermost-operator/blob/master/config/crd/bases/installation.mattermost.com_mattermosts.yaml",
            "https://github.com/mattermost/mattermost-operator/blob/master/config/crd/bases/mattermost.com_clusterinstallations.yaml",
            "https://github.com/mattermost/mattermost-operator/blob/master/config/crd/bases/mattermost.com_mattermostrestoredbs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "metacontroller/metacontroller",
        license: APACHE_V2,
        urls: &[
            "https://github.com/metacontroller/metacontroller/blob/master/manifests/production/metacontroller-crds-v1.yaml",
        ],
    },
    UpstreamSource {
        project_name: "metal3-io/baremetal-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/metal3-io/baremetal-operator/blob/main/config/crd/bases/metal3.io_baremetalhosts.yaml",
            "https://github.com/metal3-io/baremetal-operator/blob/main/config/crd/bases/metal3.io_bmceventsubscriptions.yaml",
            "https://github.com/metal3-io/baremetal-operator/blob/main/config/crd/bases/metal3.io_firmwareschemas.yaml",
            "https://github.com/metal3-io/baremetal-operator/blob/main/config/crd/bases/metal3.io_hardwaredata.yaml",
            "https://github.com/metal3-io/baremetal-operator/blob/main/config/crd/bases/metal3.io_hostfirmwaresettings.yaml",
            "https://github.com/metal3-io/baremetal-operator/blob/main/config/crd/bases/metal3.io_preprovisioningimages.yaml",
        ],
    },
    UpstreamSource {
        project_name: "microcks/microcks-ansible-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/microcks/microcks-ansible-operator/blob/master/deploy/crds/microcks_v1alpha1_microcksinstall_crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "minio/operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/minio/operator/blob/master/resources/base/crds/minio.min.io_tenants.yaml",
        ],
    },
    UpstreamSource {
        project_name: "mittwald/kubernetes-secret-generator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/mittwald/kubernetes-secret-generator/blob/master/deploy/crds/secretgenerator.mittwald.de_basicauths_crd.yaml",
            "https://github.com/mittwald/kubernetes-secret-generator/blob/master/deploy/crds/secretgenerator.mittwald.de_sshkeypairs_crd.yaml",
            "https://github.com/mittwald/kubernetes-secret-generator/blob/master/deploy/crds/secretgenerator.mittwald.de_stringsecrets_crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "NetApp/trident",
        license: APACHE_V2,
        urls: &[
            "https://github.com/NetApp/trident/blob/master/deploy/crds/trident.netapp.io_tridentorchestrators_crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "nginxinc/kubernetes-ingress",
        license: APACHE_V2,
        urls: &[
            "https://github.com/nginxinc/kubernetes-ingress/blob/main/config/crd/bases/appprotect.f5.com_aplogconfs.yaml",
            "https://github.com/nginxinc/kubernetes-ingress/blob/main/config/crd/bases/appprotect.f5.com_appolicies.yaml",
            "https://github.com/nginxinc/kubernetes-ingress/blob/main/config/crd/bases/appprotect.f5.com_apusersigs.yaml",
            "https://github.com/nginxinc/kubernetes-ingress/blob/main/config/crd/bases/appprotectdos.f5.com_apdoslogconfs.yaml",
            "https://github.com/nginxinc/kubernetes-ingress/blob/main/config/crd/bases/appprotectdos.f5.com_apdospolicy.yaml",
            "https://github.com/nginxinc/kubernetes-ingress/blob/main/config/crd/bases/appprotectdos.f5.com_dosprotectedresources.yaml",
            "https://github.com/nginxinc/kubernetes-ingress/blob/main/config/crd/bases/externaldns.nginx.org_dnsendpoints.yaml",
            "https://github.com/nginxinc/kubernetes-ingress/blob/main/config/crd/bases/k8s.nginx.org_globalconfigurations.yaml",
            "https://github.com/nginxinc/kubernetes-ingress/blob/main/config/crd/bases/k8s.nginx.org_policies.yaml",
            "https://github.com/nginxinc/kubernetes-ingress/blob/main/config/crd/bases/k8s.nginx.org_transportservers.yaml",
            "https://github.com/nginxinc/kubernetes-ingress/blob/main/config/crd/bases/k8s.nginx.org_virtualserverroutes.yaml",
            "https://github.com/nginxinc/kubernetes-ingress/blob/main/config/crd/bases/k8s.nginx.org_virtualservers.yaml",
        ],
    },
    UpstreamSource {
        project_name: "nginxinc/nginx-kubernetes-gateway",
        license: APACHE_V2,
        urls: &[
            "https://github.com/nginxinc/nginx-kubernetes-gateway/blob/main/deploy/manifests/crds/gateway.nginx.org_nginxgateways.yaml",
        ],
    },
    UpstreamSource {
        project_name: "opdev/synapse-helm",
        license: APACHE_V2,
        urls: &[
            "https://github.com/opdev/synapse-helm/blob/master/config/crd/bases/charts.opdev.io_synapses.yaml",
        ],
    },
    UpstreamSource {
        project_name: "open-cluster-management-io/registration-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/open-cluster-management-io/registration-operator/blob/main/deploy/cluster-manager/config/crds/0000_01_operator.open-cluster-management.io_clustermanagers.crd.yaml",
            "https://github.com/open-cluster-management-io/registration-operator/blob/main/deploy/klusterlet/config/crds/0000_00_operator.open-cluster-management.io_klusterlets.crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "open-feature/open-feature-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/open-feature/open-feature-operator/blob/main/config/crd/bases/core.openfeature.dev_featureflagconfigurations.yaml",
        ],
    },
    UpstreamSource {
        project_name: "open-policy-agent/gatekeeper",
        license: APACHE_V2,
        urls: &[
            "https://github.com/open-policy-agent/gatekeeper/blob/master/charts/gatekeeper/crds/assign-customresourcedefinition.yaml",
            "https://github.com/open-policy-agent/gatekeeper/blob/master/charts/gatekeeper/crds/assignimage-customresourcedefinition.yaml",
            "https://github.com/open-policy-agent/gatekeeper/blob/master/charts/gatekeeper/crds/assignmetadata-customresourcedefinition.yaml",
            "https://github.com/open-policy-agent/gatekeeper/blob/master/charts/gatekeeper/crds/config-customresourcedefinition.yaml",
            "https://github.com/open-policy-agent/gatekeeper/blob/master/charts/gatekeeper/crds/constraintpodstatus-customresourcedefinition.yaml",
            "https://github.com/open-policy-agent/gatekeeper/blob/master/charts/gatekeeper/crds/constrainttemplate-customresourcedefinition.yaml",
            "https://github.com/open-policy-agent/gatekeeper/blob/master/charts/gatekeeper/crds/constrainttemplatepodstatus-customresourcedefinition.yaml",
            "https://github.com/open-policy-agent/gatekeeper/blob/master/charts/gatekeeper/crds/expansiontemplate-customresourcedefinition.yaml",
            "https://github.com/open-policy-agent/gatekeeper/blob/master/charts/gatekeeper/crds/expansiontemplatepodstatus-customresourcedefinition.yaml",
            "https://github.com/open-policy-agent/gatekeeper/blob/master/charts/gatekeeper/crds/modifyset-customresourcedefinition.yaml",
            "https://github.com/open-policy-agent/gatekeeper/blob/master/charts/gatekeeper/crds/mutatorpodstatus-customresourcedefinition.yaml",
            "https://github.com/open-policy-agent/gatekeeper/blob/master/charts/gatekeeper/crds/provider-customresourcedefinition.yaml",
        ],
    },
    UpstreamSource {
        project_name: "open-telemetry/opentelemetry-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/open-telemetry/opentelemetry-operator/blob/main/config/crd/bases/opentelemetry.io_instrumentations.yaml",
            "https://github.com/open-telemetry/opentelemetry-operator/blob/main/config/crd/bases/opentelemetry.io_opentelemetrycollectors.yaml",
        ],
    },
    UpstreamSource {
        project_name: "openshift/hive",
        license: APACHE_V2,
        urls: &[
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_checkpoints.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_clusterclaims.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_clusterdeploymentcustomizations.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_clusterdeployments.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_clusterdeprovisions.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_clusterimagesets.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_clusterpools.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_clusterprovisions.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_clusterrelocates.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_clusterstates.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hiveinternal.openshift.io_clustersyncleases.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_dnszones.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_hiveconfigs.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_machinepoolnameleases.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_machinepools.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_selectorsyncidentityproviders.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_selectorsyncsets.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_syncidentityproviders.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hive.openshift.io_syncsets.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hiveinternal.openshift.io_clustersyncs.yaml",
            "https://github.com/openshift/hive/blob/master/config/crds/hiveinternal.openshift.io_fakeclusterinstalls.yaml",
        ],
    },
    UpstreamSource {
        project_name: "otterize/helm-charts",
        license: APACHE_V2,
        urls: &[
            "https://github.com/otterize/helm-charts/blob/main/intents-operator/crds/clientintents-customresourcedefinition.yaml",
            "https://github.com/otterize/helm-charts/blob/main/intents-operator/crds/kafkaserverconfigs-customresourcedefinition.yaml",
            "https://github.com/otterize/helm-charts/blob/main/intents-operator/crds/protectedservices-customresourcedefinition.yaml",
        ],
    },
    UpstreamSource {
        project_name: "projectcalico/calico",
        license: APACHE_V2,
        urls: &[
            "https://github.com/projectcalico/calico/blob/master/manifests/crds.yaml",
            "https://github.com/projectcalico/calico/blob/master/charts/tigera-operator/crds/operator.tigera.io_apiservers_crd.yaml",
            "https://github.com/projectcalico/calico/blob/master/charts/tigera-operator/crds/operator.tigera.io_imagesets_crd.yaml",
            "https://github.com/projectcalico/calico/blob/master/charts/tigera-operator/crds/operator.tigera.io_installations_crd.yaml",
            "https://github.com/projectcalico/calico/blob/master/charts/tigera-operator/crds/operator.tigera.io_tigerastatuses_crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "prometheus-operator/prometheus-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/prometheus-operator/prometheus-operator/blob/main/example/prometheus-operator-crd-full/monitoring.coreos.com_alertmanagerconfigs.yaml",
            "https://github.com/prometheus-operator/prometheus-operator/blob/main/example/prometheus-operator-crd-full/monitoring.coreos.com_alertmanagers.yaml",
            "https://github.com/prometheus-operator/prometheus-operator/blob/main/example/prometheus-operator-crd-full/monitoring.coreos.com_podmonitors.yaml",
            "https://github.com/prometheus-operator/prometheus-operator/blob/main/example/prometheus-operator-crd-full/monitoring.coreos.com_probes.yaml",
            "https://github.com/prometheus-operator/prometheus-operator/blob/main/example/prometheus-operator-crd-full/monitoring.coreos.com_prometheusagents.yaml",
            "https://github.com/prometheus-operator/prometheus-operator/blob/main/example/prometheus-operator-crd-full/monitoring.coreos.com_prometheuses.yaml",
            "https://github.com/prometheus-operator/prometheus-operator/blob/main/example/prometheus-operator-crd-full/monitoring.coreos.com_prometheusrules.yaml",
            "https://github.com/prometheus-operator/prometheus-operator/blob/main/example/prometheus-operator-crd-full/monitoring.coreos.com_scrapeconfigs.yaml",
            "https://github.com/prometheus-operator/prometheus-operator/blob/main/example/prometheus-operator-crd-full/monitoring.coreos.com_servicemonitors.yaml",
            "https://github.com/prometheus-operator/prometheus-operator/blob/main/example/prometheus-operator-crd-full/monitoring.coreos.com_thanosrulers.yaml",
        ],
    },
    UpstreamSource {
        project_name: "pulp/pulp-operator",
        license: GPL_V2_OR_LATER,
        urls: &[
            "https://github.com/pulp/pulp-operator/blob/main/config/crd/bases/repo-manager.pulpproject.org_pulpbackups.yaml",
            "https://github.com/pulp/pulp-operator/blob/main/config/crd/bases/repo-manager.pulpproject.org_pulprestores.yaml",
            "https://github.com/pulp/pulp-operator/blob/main/config/crd/bases/repo-manager.pulpproject.org_pulps.yaml",
        ],
    },
    UpstreamSource {
        project_name: "quay/container-security-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/quay/container-security-operator/blob/master/bundle/manifests/imagemanifestvulns.secscan.quay.redhat.com.crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "quay/quay-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/quay/quay-operator/blob/master/config/crd/bases/quay.redhat.com_quayregistries.yaml",
        ],
    },
    UpstreamSource {
        project_name: "ray-project/kuberay",
        license: APACHE_V2,
        urls: &[
            "https://github.com/ray-project/kuberay/blob/master/ray-operator/config/crd/bases/ray.io_rayclusters.yaml",
            "https://github.com/ray-project/kuberay/blob/master/ray-operator/config/crd/bases/ray.io_rayjobs.yaml",
            "https://github.com/ray-project/kuberay/blob/master/ray-operator/config/crd/bases/ray.io_rayservices.yaml",
        ],
    },
    UpstreamSource {
        project_name: "redhat-cop/namespace-configuration-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/redhat-cop/namespace-configuration-operator/blob/master/config/crd/bases/redhatcop.redhat.io_groupconfigs.yaml",
            "https://github.com/redhat-cop/namespace-configuration-operator/blob/master/config/crd/bases/redhatcop.redhat.io_namespaceconfigs.yaml",
            "https://github.com/redhat-cop/namespace-configuration-operator/blob/master/config/crd/bases/redhatcop.redhat.io_userconfigs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "redhat-cop/patch-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/redhat-cop/patch-operator/blob/main/config/crd/bases/redhatcop.redhat.io_patches.yaml",
        ],
    },
    UpstreamSource {
        project_name: "redhat-developer/service-binding-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/redhat-developer/service-binding-operator/blob/master/config/crd/bases/binding.operators.coreos.com_bindablekinds.yaml",
            "https://github.com/redhat-developer/service-binding-operator/blob/master/config/crd/bases/binding.operators.coreos.com_servicebindings.yaml",
            "https://github.com/redhat-developer/service-binding-operator/blob/master/config/crd/bases/servicebinding.io_clusterworkloadresourcemappings.yaml",
            "https://github.com/redhat-developer/service-binding-operator/blob/master/config/crd/bases/servicebinding.io_servicebindings.yaml",
        ],
    },
    UpstreamSource {
        project_name: "redhat-performance/cluster-impairment-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/redhat-performance/cluster-impairment-operator/blob/main/config/crd/bases/apps.redhat.com_clusterimpairments.yaml",
        ],
    },
    UpstreamSource {
        project_name: "RedisLabs/redis-enterprise-k8s-docs",
        license: APACHE_V2,
        urls: &[
            "https://github.com/RedisLabs/redis-enterprise-k8s-docs/blob/master/crds/reaadb_crd.yaml",
            "https://github.com/RedisLabs/redis-enterprise-k8s-docs/blob/master/crds/rec_crd.yaml",
            "https://github.com/RedisLabs/redis-enterprise-k8s-docs/blob/master/crds/redb_crd.yaml",
            "https://github.com/RedisLabs/redis-enterprise-k8s-docs/blob/master/crds/rerc_crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "rook/rook",
        license: APACHE_V2,
        urls: &[
            "https://github.com/rook/rook/blob/master/deploy/examples/crds.yaml",
        ],
    },
    UpstreamSource {
        project_name: "schemahero/schemahero",
        license: APACHE_V2,
        urls: &[
            "https://github.com/schemahero/schemahero/blob/main/config/crds/v1/databases.schemahero.io_databases.yaml",
            "https://github.com/schemahero/schemahero/blob/main/config/crds/v1/schemas.schemahero.io_datatypes.yaml",
            "https://github.com/schemahero/schemahero/blob/main/config/crds/v1/schemas.schemahero.io_migrations.yaml",
            "https://github.com/schemahero/schemahero/blob/main/config/crds/v1/schemas.schemahero.io_tables.yaml",
        ],
    },
    UpstreamSource {
        project_name: "scylladb/scylla-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/scylladb/scylla-operator/blob/master/pkg/api/scylla/v1/scylla.scylladb.com_scyllaclusters.yaml",
            "https://github.com/scylladb/scylla-operator/blob/master/pkg/api/scylla/v1alpha1/scylla.scylladb.com_nodeconfigs.yaml",
            "https://github.com/scylladb/scylla-operator/blob/master/pkg/api/scylla/v1alpha1/scylla.scylladb.com_scyllaoperatorconfigs.yaml",
        ],
    },
    UpstreamSource {
        project_name: "sematext/sematext-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/sematext/sematext-operator/blob/master/deploy/crds/sematext_v1_sematextagent_crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "sigstore/sigstore-helm-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/sigstore/sigstore-helm-operator/blob/main/config/crd/bases/helm.sigstore.dev_rekors.yaml",
        ],
    },
    UpstreamSource {
        project_name: "smartxworks/virtink",
        license: APACHE_V2,
        urls: &[
            "https://github.com/smartxworks/virtink/blob/main/deploy/crd/virt.virtink.smartx.com_virtualmachinemigrations.yaml",
            "https://github.com/smartxworks/virtink/blob/main/deploy/crd/virt.virtink.smartx.com_virtualmachines.yaml",
        ],
    },
    UpstreamSource {
        project_name: "snyk/kubernetes-monitor",
        license: APACHE_V2,
        urls: &[
            "https://github.com/snyk/kubernetes-monitor/blob/staging/snyk-operator/deploy/olm-catalog/snyk-operator/0.0.0/snykmonitors.charts.helm.k8s.io.crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "solo-io/gloo",
        license: MIT,
        urls: &[
            "https://github.com/solo-io/gloo/blob/master/install/helm/gloo/crds/enterprise.gloo.solo.io_v1_AuthConfig.yaml",
            "https://github.com/solo-io/gloo/blob/master/install/helm/gloo/crds/gateway.solo.io_v1_Gateway.yaml",
            "https://github.com/solo-io/gloo/blob/master/install/helm/gloo/crds/gateway.solo.io_v1_MatchableHttpGateway.yaml",
            "https://github.com/solo-io/gloo/blob/master/install/helm/gloo/crds/gateway.solo.io_v1_RouteOption.yaml",
            "https://github.com/solo-io/gloo/blob/master/install/helm/gloo/crds/gateway.solo.io_v1_RouteTable.yaml",
            "https://github.com/solo-io/gloo/blob/master/install/helm/gloo/crds/gateway.solo.io_v1_VirtualHostOption.yaml",
            "https://github.com/solo-io/gloo/blob/master/install/helm/gloo/crds/gateway.solo.io_v1_VirtualService.yaml",
            "https://github.com/solo-io/gloo/blob/master/install/helm/gloo/crds/gloo.solo.io_v1_Proxy.yaml",
            "https://github.com/solo-io/gloo/blob/master/install/helm/gloo/crds/gloo.solo.io_v1_Settings.yaml",
            "https://github.com/solo-io/gloo/blob/master/install/helm/gloo/crds/gloo.solo.io_v1_UpstreamGroup.yaml",
            "https://github.com/solo-io/gloo/blob/master/install/helm/gloo/crds/gloo.solo.io_v1_Upstream.yaml",
            "https://github.com/solo-io/gloo/blob/master/install/helm/gloo/crds/graphql.gloo.solo.io_v1beta1_GraphQLApi.yaml",
        ],
    },
    UpstreamSource {
        project_name: "strimzi/strimzi-kafka-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/strimzi/strimzi-kafka-operator/blob/main/helm-charts/helm3/strimzi-kafka-operator/crds/040-Crd-kafka.yaml",
            "https://github.com/strimzi/strimzi-kafka-operator/blob/main/helm-charts/helm3/strimzi-kafka-operator/crds/041-Crd-kafkaconnect.yaml",
            "https://github.com/strimzi/strimzi-kafka-operator/blob/main/helm-charts/helm3/strimzi-kafka-operator/crds/042-Crd-strimzipodset.yaml",
            "https://github.com/strimzi/strimzi-kafka-operator/blob/main/helm-charts/helm3/strimzi-kafka-operator/crds/043-Crd-kafkatopic.yaml",
            "https://github.com/strimzi/strimzi-kafka-operator/blob/main/helm-charts/helm3/strimzi-kafka-operator/crds/044-Crd-kafkauser.yaml",
            "https://github.com/strimzi/strimzi-kafka-operator/blob/main/helm-charts/helm3/strimzi-kafka-operator/crds/045-Crd-kafkamirrormaker.yaml",
            "https://github.com/strimzi/strimzi-kafka-operator/blob/main/helm-charts/helm3/strimzi-kafka-operator/crds/046-Crd-kafkabridge.yaml",
            "https://github.com/strimzi/strimzi-kafka-operator/blob/main/helm-charts/helm3/strimzi-kafka-operator/crds/047-Crd-kafkaconnector.yaml",
            "https://github.com/strimzi/strimzi-kafka-operator/blob/main/helm-charts/helm3/strimzi-kafka-operator/crds/048-Crd-kafkamirrormaker2.yaml",
            "https://github.com/strimzi/strimzi-kafka-operator/blob/main/helm-charts/helm3/strimzi-kafka-operator/crds/049-Crd-kafkarebalance.yaml",
        ],
    },
    UpstreamSource {
        project_name: "superedge/superedge",
        license: APACHE_V2,
        urls: &[
            "https://github.com/superedge/superedge/blob/main/pkg/site-manager/crd/site.superedge.io_nodegroups.yaml",
            "https://github.com/superedge/superedge/blob/main/pkg/site-manager/crd/site.superedge.io_nodeunits.yaml",
        ],
    },
    UpstreamSource {
        project_name: "tektoncd/operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/tektoncd/operator/blob/main/config/base/300-operator_v1alpha1_chain_crd.yaml",
            "https://github.com/tektoncd/operator/blob/main/config/base/300-operator_v1alpha1_config_crd.yaml",
            "https://github.com/tektoncd/operator/blob/main/config/base/300-operator_v1alpha1_hub_crd.yaml",
            "https://github.com/tektoncd/operator/blob/main/config/base/300-operator_v1alpha1_installer_set_crd.yaml",
            "https://github.com/tektoncd/operator/blob/main/config/base/300-operator_v1alpha1_pipeline_crd.yaml",
            "https://github.com/tektoncd/operator/blob/main/config/base/300-operator_v1alpha1_trigger_crd.yaml",
        ],
    },
    UpstreamSource {
        project_name: "traefik/traefik",
        license: MIT,
        urls: &[
            "https://github.com/traefik/traefik/blob/master/docs/content/reference/dynamic-configuration/traefik.io_ingressroutes.yaml",
            "https://github.com/traefik/traefik/blob/master/docs/content/reference/dynamic-configuration/traefik.io_ingressroutetcps.yaml",
            "https://github.com/traefik/traefik/blob/master/docs/content/reference/dynamic-configuration/traefik.io_ingressrouteudps.yaml",
            "https://github.com/traefik/traefik/blob/master/docs/content/reference/dynamic-configuration/traefik.io_middlewares.yaml",
            "https://github.com/traefik/traefik/blob/master/docs/content/reference/dynamic-configuration/traefik.io_middlewaretcps.yaml",
            "https://github.com/traefik/traefik/blob/master/docs/content/reference/dynamic-configuration/traefik.io_serverstransports.yaml",
            "https://github.com/traefik/traefik/blob/master/docs/content/reference/dynamic-configuration/traefik.io_serverstransporttcps.yaml",
            "https://github.com/traefik/traefik/blob/master/docs/content/reference/dynamic-configuration/traefik.io_tlsoptions.yaml",
            "https://github.com/traefik/traefik/blob/master/docs/content/reference/dynamic-configuration/traefik.io_tlsstores.yaml",
            "https://github.com/traefik/traefik/blob/master/docs/content/reference/dynamic-configuration/traefik.io_traefikservices.yaml",
        ],
    },
    UpstreamSource {
        project_name: "volcano-sh/volcano",
        license: APACHE_V2,
        urls: &[
            "https://github.com/volcano-sh/volcano/blob/master/config/crd/volcano/bases/batch.volcano.sh_jobs.yaml",
            "https://github.com/volcano-sh/volcano/blob/master/config/crd/volcano/bases/bus.volcano.sh_commands.yaml",
            "https://github.com/volcano-sh/volcano/blob/master/config/crd/volcano/bases/nodeinfo.volcano.sh_numatopologies.yaml",
            "https://github.com/volcano-sh/volcano/blob/master/config/crd/volcano/bases/scheduling.volcano.sh_podgroups.yaml",
            "https://github.com/volcano-sh/volcano/blob/master/config/crd/volcano/bases/scheduling.volcano.sh_queues.yaml",
            "https://github.com/volcano-sh/volcano/blob/master/config/crd/jobflow/bases/flow.volcano.sh_jobflows.yaml",
            "https://github.com/volcano-sh/volcano/blob/master/config/crd/jobflow/bases/flow.volcano.sh_jobtemplates.yaml",
        ],
    },
    UpstreamSource {
        project_name: "wildfly/wildfly-operator",
        license: APACHE_V2,
        urls: &[
            "https://github.com/wildfly/wildfly-operator/blob/main/config/crd/bases/wildfly.org_wildflyservers.yaml",
        ],
    },
    UpstreamSource {
        project_name: "zalando/postgres-operator",
        license: MIT,
        urls: &[
            "https://github.com/zalando/postgres-operator/blob/master/charts/postgres-operator/crds/postgresqls.yaml",
            "https://github.com/zalando/postgres-operator/blob/master/charts/postgres-operator/crds/operatorconfigurations.yaml",
            "https://github.com/zalando/postgres-operator/blob/master/charts/postgres-operator/crds/postgresteams.yaml",
        ],
    },
];

pub struct UpstreamSource<'a> {
    pub project_name: &'a str,
    pub license: &'a str,
    pub urls: &'a [&'a str],
}

const APACHE_V2: &'static str = "Apache-2.0";
const MIT: &'static str = "MIT";
const ELASTIC: &'static str = "LicenseRef-Elastic";
const AQUA: &'static str = "LicenseRef-Aqua";
const FLAGSMITH: &'static str = "LicenseRef-Flagsmith";
const EPL_V2: &'static str = "EPL-2.0";
const AGPL_V3_ONLY: &'static str = "AGPL-3.0-only";
const GPL_V2_OR_LATER: &'static str = "GPL-2.0-or-later";
const UPL_V1: &'static str = "UPL-1.0";
