function getFirstH1(doc)
    for _, block in ipairs(doc.blocks) do
        if block.t == "Header" and block.level == 1 then
            return pandoc.utils.stringify(block.content)
        end
    end
    -- fallback to the filename if no h1 is found
    return pandoc.utils.stringify(doc.meta["basepath"])
end

function Pandoc(doc)
  local entries = {}

  -- NB: it looks like list_directory may return entries in an undefined order
  for _, path in ipairs(pandoc.system.list_directory("posts")) do
    local year, month, day = path:match("(%d+)-(%d+)-(%d+)-.*.md")
    local timestamp = os.time({year = year, month = month, day = day})
    local formattedDate = os.date("%b %Y", timestamp)

    local postPandoc =
      pandoc.read(io.open("posts/" .. path):read("*all"), "gfm")

    local title = getFirstH1(postPandoc)

    table.insert(
        entries,
        { timestamp = timestamp,
          md = string.format("  * <div class=\"post-date\">%s</div> [%s](%s)",
                      formattedDate,
                      title,
                      path:match("(.*).md"))})
  end

  table.sort(entries, function (a, b) return a["timestamp"] > b["timestamp"] end)

  md_list = {}
  for _, entry in ipairs(entries) do
    table.insert(md_list, entry["md"])
  end

  local blocks = pandoc.read(
    "<div class=\"post-list\">" .. table.concat(md_list, "\n") .. "</div>",
    "markdown")

  for _, block in ipairs(blocks.blocks) do
    table.insert(doc.blocks, block)
  end

  return doc
end
