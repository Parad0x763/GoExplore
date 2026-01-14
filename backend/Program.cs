using Swashbuckle.AspNetCore.SwaggerGen;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
// Learn more about configuring OpenAPI at https://aka.ms/aspnet/openapi
builder.Services.AddOpenApi();
builder.Services.AddSwaggerGen();
builder.Services.AddControllers();
builder.Services.AddCors(options => {
    options.AddPolicy("AllowFrontend",
        b => b // :5173 is default Vite app port
            .WithOrigins("http://localhost:5173") // TODO: move this value into appsettings.json
            // TODO: add functionality for the appsettings.Development.json to be loaded in dev
            .AllowAnyHeader()
            .AllowAnyMethod()
    );
});

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment()) {
    app.MapOpenApi();
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();

app.Run();
