using Swashbuckle.AspNetCore.SwaggerGen;

 WebApplicationBuilder? builder = WebApplication.CreateBuilder(args);

// The Environment for development is set in ./Properties/launchSettings.json
IHostEnvironment env = builder.Environment;
builder.Configuration
    .AddJsonFile($"appsettings.{env.EnvironmentName}.json", optional: false, reloadOnChange: true);

// Add services to the container.
// Learn more about configuring OpenAPI at https://aka.ms/aspnet/openapi
builder.Services.AddOpenApi();
builder.Services.AddSwaggerGen();
builder.Services.AddControllers();
builder.Services.AddCors(options => {
    options.AddPolicy("AllowFrontend",
        b => b
            .WithOrigins(builder.Configuration.GetSection("FrontendHost").Value ?? "")
            .AllowAnyHeader()
            .AllowAnyMethod()
    );
});

WebApplication? app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment()) {
    app.MapOpenApi();
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();

app.Run();
