function Pandoc(doc)
  local year, month, day = doc.meta["basepath"]:match("(%d+)-(%d+)-(%d+)-.*")
  local timestamp = os.time({year = year, month = month, day = day})

  table.insert(doc.blocks, 2, pandoc.Para(os.date("%Y %b %d", timestamp)))
  table.insert(doc.blocks, 3, pandoc.HorizontalRule{})

  return doc
end
