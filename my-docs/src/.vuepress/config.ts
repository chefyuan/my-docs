import { defineUserConfig } from "vuepress";
import { googleAnalyticsPlugin } from '@vuepress/plugin-google-analytics'
import { readmorePlugin } from 'vuepress-plugin-readmore-popular-next'
import theme from "./theme.js";
import { blog } from "vuepress-theme-hope";
import { registerComponentsPlugin } from '@vuepress/plugin-register-components'
import { getDirname, path } from '@vuepress/utils'
import { sitemapPlugin } from "vuepress-plugin-sitemap2";

// import { FooBar } from "./components/FooBar.vue";

const __dirname = getDirname(import.meta.url);

export default defineUserConfig({
  base: "/",
  shouldPrefetch: false,

  locales: {
    "/": {
      lang: "zh-CN",
      title: "程序厨",
      description: "酷爱做饭的程序员",

      head:
        [['link', { rel: 'icon', href: '/images/logo.jpg' }],
        // ["script", { src: "/scripts/demo.js"}],
        ['meta', { name: 'baidu-site-verification', content: 'codeva-t9fUvv6fEU' }],
        [
          "script",
          {},
          `
        var _hmt = _hmt || [];
        (function() {
          var hm = document.createElement("script");
          hm.src = "https://hm.baidu.com/hm.js?e3f7929fae135765220efa9e8bfe2d7a";
          var s = document.getElementsByTagName("script")[0]; 
          s.parentNode.insertBefore(hm, s);
        })();
          `
        ]],

    },


  },

  // alias: { "@theme-hope/components/FooterInfo": path.resolve(__dirname, "./components/FooterInfo.vue"),
  // },
  alias: {
    "@theme-hope/components/NormalPage": path.resolve(__dirname, "./components/NormalPage.vue")
  },

  port: 80,
  theme,

  plugins: [



    // registerComponentsPlugin({
    //   components: {
    //     FooBar: path.resolve(__dirname, './components/FooBar.vue'),

    //   },
    // }),
    sitemapPlugin({
      hostname: 'www.chengxuchu.com',
    }),

    readmorePlugin({
      // 已申请的博客 ID
      blogId: '67962-1834575117932-974',
      // 已申请的微信公众号名称
      name: '算法基地',
      // 已申请的微信公众号回复关键词
      keyword: '验证码',
      // 已申请的微信公众号二维码图片
      qrcode: 'https://chengxuchu-1301103198.cos.ap-beijing.myqcloud.com/Photo/202411222340371.png',
      // 文章内容的 JS 选择器，若使用的不是官方默认主题，则需要根据第三方的主题来设置
      selector: 'div.theme-hope-content',
      // 自定义的 JS 资源链接，可用于 CDN 加速
      // 文章排除添加引流工具的 URL 规则，支持使用路径、通配符、正则表达式的匹配规则
      excludes: { strExp: [], regExp: ['/team/*', '/README/*', '/base/*', '/timeline/*'] },
      // 是否反转 URL 排除规则的配置，即只有符合排除规则的文章才会添加引流工具
      reverse: false,
      // 文章内容的预览高度
      height: 'auto',
      // 文章解锁后凭证的有效天数
      expires: 365,
      // 定时校验凭证有效性的时间间隔（秒）
      interval: 300,
      // 每篇文章随机添加引流工具的概率，有效范围在 0.1 ~ 1 之间，1 则表示所有文章默认都自动添加引流工具
      random: 1

    }),
    googleAnalyticsPlugin({
      id: "G-B2BW1T1C3G",
      debug: true,
    }),



  ]
});
