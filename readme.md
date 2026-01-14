# GoExplore

## .NET

- `dotnet new sln`
- After `backend` and `backend.testing`
    + `dotnet sln add backend/backend.csproj`
    + `dotnet sln add backend.testing/backend.testing.csproj`

### backend

- `cd backend`
- `dotnet new webapi -o .`
- `dotnet add package Microsoft.EntityFrameworkCore`
- `dotnet package add Microsoft.EntityFrameworkCore.Design`
- `dotnet package add Npgsql.EntityFrameworkCore.PostgreSQL`
- `dotnet package add Swashbuckle.AspNetCore.SwaggerGen`
- `dotnet package add Swashbuckle.AspNetCore`

#### JSON Configuration Provider

- [JSON-Config-Provider](https://learn.microsoft.com/en-us/dotnet/core/extensions/configuration-providers#:~:text=JSON%20configuration%20provider,-The)

### backend.testing

- `cd backend.testing`
- `dotnet new mstest -o .`
- [MSTest Unit Testing](https://learn.microsoft.com/en-us/dotnet/core/testing/unit-testing-csharp-with-mstest)

## Preact

- `cd frontend`
- `npm install -g vite@latest # install vite globally for npm`
- `npm create vite@latest`
    + Name -> `.`
    + Choose `Preact` -> `TypeScript`