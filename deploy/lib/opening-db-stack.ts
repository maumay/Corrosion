import * as cdk from '@aws-cdk/core';
import * as db from '@aws-cdk/aws-dynamodb';

export interface OpeningDatabaseStackProps extends cdk.StackProps {
  readonly openingsTableName: string,
  readonly positionAttributeName: string
  readonly readCapacity: number
  readonly writeCapacity: number
}

export class MyopicDatabaseStack extends cdk.Stack {

  constructor(scope: cdk.Construct, id: string, props: OpeningDatabaseStackProps) {
    super(scope, id, props);

    new db.Table(this, `${id}-Openings`, {
      tableName: props.openingsTableName,
      billingMode: db.BillingMode.PROVISIONED,
      readCapacity: props.readCapacity,
      writeCapacity: props.writeCapacity,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      partitionKey: {
        name: props.positionAttributeName,
        type: db.AttributeType.STRING,
      },
    })
  }

}