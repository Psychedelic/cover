import type { AWS } from '@serverless/typescript';

import { publish, consume } from '@functions'

const serverlessConfiguration: AWS = {
  service: 'cover',
  frameworkVersion: '2',
  custom: {
    webpack: {
      webpackConfig: './webpack.config.js',
      includeModules: true,
    },
  },
  plugins: [
    'serverless-webpack', 
    'serverless-offline',
  ],
  provider: {
    name: 'aws',
    runtime: 'nodejs14.x',
    region: 'us-west-2',
    apiGateway: {
      minimumCompressionSize: 1024,
      shouldStartNameWithService: true,
    },
    environment: {
      AWS_NODEJS_CONNECTION_REUSE_ENABLED: '1',
      QUEUE_URL: { Ref: 'CoverQueue' },
    },
    lambdaHashingVersion: '20201221',
  },
  functions: { publish, consume },
  resources: {
    Resources: {
      // ToDo: {botch} add vpc
      CoverQueue: {
        Type: 'AWS::SQS::Queue',
        Properties: {
          MessageRetentionPeriod: 300,
          RedrivePolicy: {
            deadLetterTargetArn: { 'Fn::GetAtt': [ 'CoverDeadLetterQueue', 'Arn']},
            maxReceiveCount: 1
          }
        },
      },
      CoverDeadLetterQueue: { 
        Type: 'AWS::SQS::Queue'
      },
    },
  },
};

module.exports = serverlessConfiguration;
