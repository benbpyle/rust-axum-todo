import { Vpc } from "aws-cdk-lib/aws-ec2";
import { Cluster, ContainerImage } from "aws-cdk-lib/aws-ecs";
import { ApplicationLoadBalancedFargateService } from "aws-cdk-lib/aws-ecs-patterns";
import { Construct } from "constructs";
import path = require("path");

export class EcsConstruct extends Construct {
    constructor(scope: Construct, id: string) {
        super(scope, id);

        const vpc = new Vpc(this, "SampleVpc", {
            maxAzs: 3, // Default is all AZs in region
        });

        const cluster = new Cluster(this, "SampleCluster", {
            vpc: vpc,
        });

        // Create a load-balanced Fargate service and make it public
        new ApplicationLoadBalancedFargateService(this, "MyFargateService", {
            cluster: cluster, // Required
            cpu: 256, // Default is 256
            desiredCount: 1, // Default is 1
            listenerPort: 8080,
            taskImageOptions: {
                image: ContainerImage.fromAsset(path.resolve(__dirname, "../")),
                environment: {},
            },
            memoryLimitMiB: 2048, // Default is 512
            publicLoadBalancer: true, // Default is true
        });
    }
}
