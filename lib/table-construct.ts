import { Construct } from "constructs";
import { AttributeType, BillingMode, Table } from "aws-cdk-lib/aws-dynamodb";
import { RemovalPolicy } from "aws-cdk-lib";

export class TableConstruct extends Construct {
    private readonly _table: Table;

    get table(): Table {
        return this._table;
    }

    constructor(scope: Construct, id: string) {
        super(scope, id);

        this._table = new Table(scope, "Table", {
            billingMode: BillingMode.PAY_PER_REQUEST,
            removalPolicy: RemovalPolicy.DESTROY,
            partitionKey: { name: "id", type: AttributeType.STRING },
            pointInTimeRecovery: false,
            tableName: "Todo",
        });

        // const repository = new ecr.Repository(this, "Repository", {
        //     repositoryName: "rust-images",
        // });

        // const sampleImage1 = new assets.DockerImageAsset(this, "SampleImage1", {
        //     // Put the directory where your Dockerfile is below
        //     directory: path.join(__dirname, "../"),
        //     invalidation: {
        //         buildArgs: false,
        //     },
        // });

        // new assets(this, "DeployDockerImage1", {
        //     src: new ecrdeploy.DockerImageName(image.imageUri),
        //     dest: new ecrdeploy.DockerImageName(
        //         `${cdk.Aws.ACCOUNT_ID}.dkr.ecr.us-west-2.amazonaws.com/my-nginx:latest`
        //     ),
        // });
    }
}
