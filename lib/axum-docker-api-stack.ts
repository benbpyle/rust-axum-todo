import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { EcsConstruct } from "./ecs-construct";
import { TableConstruct } from "./table-construct";
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class AxumDockerApiStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        // const ecrConstruct = new EcrConstruct(this, "EcrConstruct");
        const tableConstruct = new TableConstruct(this, "TableConstruct");
        const ecsConstruct = new EcsConstruct(this, "EcsConstruct");
    }
}
