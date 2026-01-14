using Microsoft.AspNetCore.Mvc;
using static Microsoft.AspNetCore.Http.StatusCodes;

namespace backend.Controllers;
public class ExampleController : ControllerBase {
    public ExampleController() { }

    [ProducesResponseType(Status204NoContent)]
    [ProducesResponseType(Status400BadRequest)]
    [Route(Routes.ExampleRoute)]
    public async Task<ActionResult> ExampleEndpoint() {
        try {
            // request came back
            return NoContent();
        } catch (Exception ex) {
            return BadRequest($"ERROR: {ex.Message}");
        }
    }
}