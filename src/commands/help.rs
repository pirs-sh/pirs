use crate::re_exports::*;

pub const HELP_COMMAND: crate::Command = crate::Command {
  name: "help",
  description: "Display help information",
  args: &[],
  handler: |ctx, _args| {
    Box::pin(async move {
      crate::logger::create_logger_from_logger!(ctx.logger, true);

      log!(raw, "Sea Shell version {}\n", crate::VERSION);

      for command in &ctx.state.commands {
        log!(raw, "{}: {}", command.name, command.description);
      }

      (Some(ctx), 0)
    })
  },
};
