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
  local year, month, day = doc.meta["basepath"]:match("(%d+)-(%d+)-(%d+)-.*")
  local timestamp = os.time({year = year, month = month, day = day})

  table.insert(doc.blocks, 2, pandoc.Para(os.date("%Y %b %d", timestamp)))
  table.insert(doc.blocks, 3, pandoc.HorizontalRule{})

  doc.meta["title"] = getFirstH1(doc)

  return doc
end
