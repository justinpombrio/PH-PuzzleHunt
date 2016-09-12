<?xml version="1.0" encoding="UTF-8"?>

<xsl:transform version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

  <xsl:strip-space elements="*"/>

  <xsl:template match="title">
    <h1><xsl:copy-of select="node()"/></h1>
  </xsl:template>

  <xsl:template match="subtitle">
    <p class="subtitle">
      <xsl:copy-of select="node()"/>
    </p>
  </xsl:template>

  <xsl:template match="content">
    <xsl:copy-of select="node()"/>
  </xsl:template>

  <xsl:template match="puzzle">
    <p class="subtitle">This is a puzzle.</p>
    <xsl:copy-of select="description/node()"/>
    <p><a href="/guess.xml?puzzle={id}">Submit an answer</a></p>
  </xsl:template>


  
  <!--******************** FORMS ********************-->

  <xsl:template name="PuzzleInput">
    <select id="puzzle-input">
      <script type="text/javascript" defer="defer">
        setupDropdown();
      </script>
      <xsl:for-each select="document('hunt.xml')//puzzle[@id]">
        <option value="{@id}">
          <xsl:value-of select="@name"/>
        </option>
      </xsl:for-each>
      <option value="" selected="selected">
        Select a puzzle
      </option>
    </select>
  </xsl:template>

  <xsl:template match="multi-form">
    <form action="{submit-button/@action}">
      <p>
        <table>
          <xsl:for-each select="input">
            <tr>
              <td><b><xsl:value-of select="@name"/>:</b></td>
              <td><xsl:value-of select="."/></td>
            </tr>
          </xsl:for-each>
        </table>
      </p>
      <p>
        <table>
          <tbody id="multi-form-rows">
            <script>
              setupMultiform();
            </script>
            <tr>
              <xsl:for-each select="input">
                <th><xsl:value-of select="normalize-space(@name)"/></th>
              </xsl:for-each>
            </tr>
            <tr id="row-template" style="display:none">
              <xsl:for-each select="input">
                <td>
                  <xsl:if test="contains(@type, 'short-text')">
                    <input type="text"
                           name="{@id}"
                           class="multi-form-cell short-text"/>
                  </xsl:if>
                  <xsl:if test="contains(@type, 'long-text')">
                    <input type="text"
                           name="{@id}"
                           class="multi-form-cell long-text"/>
                  </xsl:if>
                  <xsl:if test="contains(@type, 'generated')">
                    <input type="text"
                           name="{@id}"
                           disabled="true"
                           class="multi-form-cell long-text"/>
                  </xsl:if>
                </td>
              </xsl:for-each>
              <td>
                <a href="#" onclick="deleteRow(this)">delete</a>
              </td>
            </tr>
          </tbody>
        </table>
        <a href="#" onclick="addRow(this)">add row</a>
      </p>
      <input type="submit" value="{normalize-space(submit-button)}"/>
    </form>
  </xsl:template>

  <xsl:template match="form">
    <form action="{submit-button/@action}">
      <xsl:for-each select="input">
        <xsl:if test="contains(@type, 'text')">
          <p>
            <xsl:value-of select="normalize-space(.)"/>:
            <br/>
            <input type="text" name="{@id}"/>
          </p>
        </xsl:if>
        <xsl:if test="contains(@type, 'puzzle')">
          <p>
            <xsl:value-of select="normalize-space(.)"/>:
            <br/>
            <xsl:call-template name="PuzzleInput"/>
            </p>
        </xsl:if>
      </xsl:for-each>
      <input type="submit" value="{normalize-space(submit-button)}"/>
    </form>
  </xsl:template>

  <xsl:template match="list-all-puzzles">
    <ul style="list-style-type: none">
      <xsl:for-each select="document('hunt.xml')/hunt/all-puzzles/*">
        <xsl:call-template name="list-a-puzzles"/>
      </xsl:for-each>
    </ul>
  </xsl:template>

  <xsl:template name="list-a-puzzles">
    <xsl:if test="name() = 'puzzles'">
      <li>
        <b><xsl:value-of select="@name"/>:</b>
        <ul>
          <xsl:for-each select="*">
            <xsl:call-template name="list-a-puzzles"/>
          </xsl:for-each>
        </ul>
      </li>
    </xsl:if>
    <xsl:if test="name() = 'puzzle'">
      <li>
        <a href="puzzles/{@id}.xml">
          <xsl:value-of select="@name"/>
        </a>
      </li>
    </xsl:if>
  </xsl:template>

  <xsl:template name="Header">
    <xsl:variable name="hunt-name" select="document('hunt.xml')/hunt/name"/>
    <xsl:variable name="puzzle-list">
      <xsl:for-each select="document('hunt.xml')//puzzle[@id]/@id">
        <xsl:value-of select="."/>
        <xsl:text>,</xsl:text>
      </xsl:for-each>
    </xsl:variable>
    <title>
      <xsl:value-of select="$hunt-name"/>
    </title>
    <link rel="stylesheet" type="text/css" href="../css/style.css"/>
    <script type="text/javascript">
      var PUZZLES = '<xsl:value-of select="$puzzle-list"/>'.split(",");
      PUZZLES.pop();
    </script>
    <script type="text/javascript" src="/ph.js"/>
  </xsl:template>

  <xsl:template match="page">
    <xsl:variable name="hunt-name" select="document('hunt.xml')/hunt/name"/>
    <html>
      <head>
        <xsl:call-template name="Header"/>
      </head>
      <body>
        <ul class="nav">
          <li class="nav1">
            <a href="/index.xml">
              <xsl:value-of select="$hunt-name"/>
            </a>
          </li>
          <li class="nav2"><a href="/team.xml">Team</a></li>
          <li class="nav3"><a href="/leaderboard.xml">Leaderboard</a></li>
          <li class="nav4"><a href="/puzzles.xml">Puzzles</a></li>
        </ul>
        <article>
          <xsl:apply-templates select="*"/>
          <footer>
            <a style="text-decoration: none"
               href="https://github.com/justinpombrio/PuzzleHunt-PH">
              * Made with Puzzle Hunt: PH *
            </a>
          </footer>
        </article>
      </body>
    </html>
  </xsl:template>
  
  <xsl:template match="master-page">
    <html>
      <head>
        <xsl:call-template name="Header"/>
      </head>
      <body>
        <ul class="nav">
          <li class="nav1"><a href="/master/hunt.xml">Hunt</a></li>
          <li class="nav2"><a href="/master/puzzles.xml">Puzzles</a></li>
          <li class="nav3"><a href="/master/hints.xml">Hints</a></li>
          <li class="nav4"><a href="/master/waves.xml">Waves</a></li>
        </ul>
        <article>
          <xsl:apply-templates select="*"/>
          <footer>
            * Welcome, master. *
          </footer>
        </article>
      </body>
    </html>
  </xsl:template>
  
</xsl:transform>
