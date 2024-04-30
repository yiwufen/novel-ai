/* global use, db */
// MongoDB Playground
// To disable this template go to Settings | MongoDB | Use Default Template For Playground.
// Make sure you are connected to enable completions and to be able to run a playground.
// Use Ctrl+Space inside a snippet or a string literal to trigger completions.
// The result of the last command run in a playground is shown on the results panel.
// By default the first 20 documents will be returned with a cursor.
// Use 'console.log()' to print to the debug output.
// For more documentation on playgrounds please refer to
// https://www.mongodb.com/docs/mongodb-vscode/playgrounds/

// Select the database to use.
use('novel');

// db.novels.findOne({novel_name: "云边有个小卖部"});

// 插入小说数据
// db.novels.insertOne({
//   novel_name: "云边有个小卖部"
// });

// var novelId = db.novels.findOne({ novel_name: "云边有个小卖部" })._id;
// Insert a few documents into the sales collection.
db.chapters.insertOne({
  novel_name: "云边有个小卖部",
  chapter_title: "第一章",
  bg_novel: "《云边有个小卖部》讲述了主角张小凡在古老村落开设小卖部的故事。",
  current_plot: "张小凡刚刚发现小卖部的地下室藏有一批古老的书籍和物件，看似与村庄的历史密切相关。",
  plot_development: "展示张小凡与村长之间的关系，揭示关于村庄历史的秘密。",
  key_role: "张小凡（好奇、理性）、村长（严肃、知识渊博）",
  writing_style: "保持故事的温馨和轻松的基调，对话自然流畅。",
  chapter_start: "请从张小凡与村长讨论地下室的发现开始。",
  content: ""
});
db.chapters.insertOne({
    novel_name: "云边有个小卖部",
    chapter_title: "第二章",
    bg_novel: "《云边有个小卖部》讲述了主角张小凡在古老村落开设小卖部的故事。",
    current_plot: "张小凡刚刚发现小卖部的地下室藏有一批古老的书籍和物件，看似与村庄的历史密切相关。",
    plot_development: "展示张小凡与村长之间的关系，揭示关于村庄历史的秘密。",
    key_role: "张小凡（好奇、理性）、村长（严肃、知识渊博）",
    writing_style: "保持故事的温馨和轻松的基调，对话自然流畅。",
    chapter_start: "请从张小凡与村长讨论地下室的发现开始。",
    content: ""
  });
// db.getCollection('novel_edits').insertOne(
//   {
//     "novel_name": "云边有个小卖部",
//     "chapter": [
//         {
//         "chapter_title": "第一章",
//         "bg_novel": "《云边有个小卖部》讲述了主角张小凡在古老村落开设小卖部的故事。张小凡原本是城市里的白领，因一次偶然的机会，他继承了祖上传下来的这个小卖部，并决定留在村里经营。",
//         "current_plot": "张小凡刚刚发现小卖部的地下室藏有一批古老的书籍和物件，看似与村庄的历史密切相关。他计划与村里的长者们探讨这些发现。",
//         "plot_development": "我希望通过对话展示张小凡与村长之间的关系，同时揭示一些关于村庄历史的秘密。",
//         "key_role": "张小凡（好奇、理性）、村长（严肃、知识渊博）。",
//         "writing_style": "保持故事的温馨和轻松的基调，对话自然流畅。",
//         "chapter_start": "请从张小凡与村长讨论地下室的发现开始。"
//         },
//         {
//         "chapter_title": "第二章",
//         "bg_novel": "《云边有个小卖部》讲述了主角张小凡在古老村落开设小卖部的故事。张小凡原本是城市里的白领，因一次偶然的机会，他继承了祖上传下来的这个小卖部，并决定留在村里经营。",
//         "current_plot": "张小凡刚刚发现小卖部的地下室藏有一批古老的书籍和物件，看似与村庄的历史密切相关。他计划与村里的长者们探讨这些发现。",
//         "plot_development": "我希望通过对话展示张小凡与村长之间的关系，同时揭示一些关于村庄历史的秘密。",
//         "key_role": "张小凡（好奇、理性）、村长（严肃、知识渊博）。",
//         "writing_style": "保持故事的温馨和轻松的基调，对话自然流畅。",
//         "chapter_start": "请从张小凡与村长讨论地下室的发现开始。"
//         }
//     ]
//   },
// );


