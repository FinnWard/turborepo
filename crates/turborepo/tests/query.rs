mod common;

#[cfg(not(windows))]
#[test]
fn test_double_symlink() -> Result<(), anyhow::Error> {
    check!(
        "oxc_repro",
        "npm@10.5.0",
        "get_dependencies" => "query {
                 file(path: \"./index.js\") {
                    path
                    dependencies {
                      files { items { path } }
                      errors { items { message import } }
                    }
                 }
              }",
    );
    Ok(())
}

#[test]
fn test_trace() -> Result<(), anyhow::Error> {
    check!(
        "turbo_trace",
        "npm@10.5.0",
        "get `main.ts`" => "query { file(path: \"main.ts\") { path } }",
        "get `main.ts` with dependencies" => "query { file(path: \"main.ts\") { path, dependencies { files { items { path } } } } }",
        "get `button.tsx` with dependencies" => "query { file(path: \"button.tsx\") { path, dependencies { files { items { path } } } } }",
        "get `circular.ts` with dependencies" => "query { file(path: \"circular.ts\") { path dependencies { files { items { path } } } } }",
        "get `invalid.ts` with dependencies" => "query { file(path: \"invalid.ts\") { path dependencies { files { items { path } } errors { items { message } } } } }",
        "get `main.ts` with ast" => "query { file(path: \"main.ts\") { path ast } }",
        "get `main.ts` with depth = 0" => "query { file(path: \"main.ts\") { path dependencies(depth: 1) { files { items { path } } } } }",
    );

    Ok(())
}
