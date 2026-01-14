using Microsoft.VisualStudio.TestTools.UnitTesting;
using Microsoft.AspNetCore.Mvc;
using backend.Controllers;

namespace backend.testing;

[TestClass]
public class TestExampleEndpoint {

    private ExampleController _exampleController;

    public TestExampleEndpoint() {
        _exampleController = new ExampleController();
    }

    [TestMethod]
    public async Task TestEndpoint() {
        NoContentResult result = await _exampleController.ExampleEndpoint() as NoContentResult;
        Console.WriteLine(result);
        Assert.AreEqual<int>(result.StatusCode, 204);
    }
}
