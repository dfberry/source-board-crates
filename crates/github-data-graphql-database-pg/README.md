# Get repo data

Nightly GitHub action and Rust app to get list of repos from postgres and insert data into mongodb.

## Get unique repo list out of Postgres

Get org and repo name from Postgres with Diesel.

## For each repo get GitHub information

```json
{
    "createdAt": "2020-07-08T14:51:56Z",
    "description": "MS Learn Node Essentials sample code",
    "diskUsage": 932,
    "forksCount": 159,
    "name": "node-essentials",
    "openIssuesCount": 2,
    "openPRsCount": 32,
    "pushedAt": "2024-12-02T13:57:36Z",
    "stargazersCount": 115,
    "updatedAt": "2024-11-30T05:39:29Z",
    "url": "https://github.com/MicrosoftDocs/node-essentials",
    "watchersCount": 19
}
```

## Insert GitHub info into MongoDB

Insert as log

```json
{
	"_id" : ObjectId("67578276c786cf62d56ea72c"),
	"name" : "azure-typescript-e2e-apps",
	"description" : "Monorepo - Apps written in TypeScript to be either 1) deployed to Azure hosting or 2) integrated with Azure services",
	"url" : "https://github.com/Azure-Samples/azure-typescript-e2e-apps",
	"createdAt" : "2023-04-24T14:20:05Z",
	"updatedAt" : "2024-12-02T08:30:52Z",
	"pushedAt" : "2024-10-29T16:14:59Z",
	"diskUsage" : 1421,
	"watchersCount" : 15,
	"stargazersCount" : 34,
	"forksCount" : 539,
	"openIssuesCount" : 1,
	"openPRsCount" : 114,
	"org" : "azure-samples",
	"repo" : "azure-typescript-e2e-apps",
	"log_time" : "2024-12-09T23:51:17.949865628+00:00"
}
```